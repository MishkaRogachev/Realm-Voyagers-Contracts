use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::state::*;

const REALM_SEED: &[u8] = b"realm";

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum RealmEventType {
    RealmCreated { name: String, description: String },
    RealmUpdated { name: String, description: String },
    RealmDeleted {},
}

#[event]
pub struct RealmEvent {
    pub event_type: RealmEventType,
    pub realm_pubkey: Pubkey,
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateRealm<'info> {
    #[account(
        init,
        payer = master,
        space = 8 + Realm::INIT_SPACE,
        seeds = [REALM_SEED, master.key().as_ref(), id.as_bytes()],
        bump
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_realm(
    ctx: Context<CreateRealm>,
    _id: String,
    name: String,
    description: String,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;
    realm.name = name.clone();
    realm.description = description.clone();
    realm.created_at = Clock::get()?.unix_timestamp;

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmCreated { name, description },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct UpdateRealm<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, master.key().as_ref(), id.as_bytes()],
        bump,
        realloc = 8 + Realm::INIT_SPACE,
        realloc::payer = master,
        realloc::zero = true
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_realm(
    ctx: Context<UpdateRealm>,
    _id: String,
    name: String,
    description: String,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, ErrorCode::NameTooLong);
    require!(
        description.len() <= MAX_DESCRIPTION_LEN,
        ErrorCode::DescriptionTooLong
    );

    let realm = &mut ctx.accounts.realm;
    realm.name = name.clone();
    realm.description = description.clone();

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmUpdated { name, description },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct DeleteRealm<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, master.key().as_ref(), id.as_bytes()],
        bump,
        close = master,
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn delete_realm(ctx: Context<DeleteRealm>, _id: String) -> Result<()> {
    emit!(RealmEvent {
        realm_pubkey: ctx.accounts.realm.key(),
        event_type: RealmEventType::RealmDeleted {},
    });

    Ok(())
}
