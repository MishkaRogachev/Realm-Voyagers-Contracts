use anchor_lang::prelude::*;

use crate::state::*;

const REALM_SEED: &[u8] = b"realm";

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum RealmEventType {
    Created { name: String, description: String },
    Updated { name: String, description: String },
    Deleted {},
}

#[event]
pub struct RealmEvent {
    pub event_type: RealmEventType,
    pub realm_pubkey: Pubkey,
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct CreateRealm<'info> {
    #[account(
        init,
        payer = realm_master,
        space = 8 + Realm::INIT_SPACE,
        seeds = [REALM_SEED, realm_master.key().as_ref(), seed.as_bytes()],
        bump
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub realm_master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_realm(
    ctx: Context<CreateRealm>,
    seed: String,
    name: String,
    description: String,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;
    realm.realm_master = ctx.accounts.realm_master.key();
    realm.seed = seed;
    realm.name = name.clone();
    realm.description = description.clone();
    realm.created_at = Clock::get()?.unix_timestamp;

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::Created { name, description }
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateRealm<'info> {
    #[account(
        mut,
        has_one = realm_master,
        seeds = [REALM_SEED, realm_master.key().as_ref(), realm.seed.as_bytes()],
        bump
    )]
    pub realm: Account<'info, Realm>,

    pub realm_master: Signer<'info>,
}

pub fn update_realm(ctx: Context<UpdateRealm>, name: String, description: String) -> Result<()> {
    let realm = &mut ctx.accounts.realm;
    realm.name = name.clone();
    realm.description = description.clone();

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::Updated { name, description }
    });

    Ok(())
}

#[derive(Accounts)]
pub struct DeleteRealm<'info> {
    #[account(
        mut,
        close = realm_master,
        has_one = realm_master,
        seeds = [REALM_SEED, realm_master.key().as_ref(), realm.seed.as_bytes()],
        bump
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub realm_master: Signer<'info>,
}

pub fn delete_realm(ctx: Context<DeleteRealm>) -> Result<()> {
    emit!(RealmEvent {
        realm_pubkey: ctx.accounts.realm.key(),
        event_type: RealmEventType::Deleted {}
    });

    Ok(())
}
