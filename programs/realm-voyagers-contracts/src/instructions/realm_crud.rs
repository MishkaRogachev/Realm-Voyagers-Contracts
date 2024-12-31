use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(realm_id: String, name: String, description: String)]
pub struct CreateRealm<'info> {
    #[account(
        init,
        payer = master,
        space = crate::realm_space!(name, description, 1),
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_realm(
    ctx: Context<CreateRealm>,
    _realm_id: String,
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
    realm.created_at = Clock::get()?.unix_timestamp;
    realm.masters.push(RealmMaster {
        pubkey: *ctx.accounts.master.key,
        role: RealmMasterRole::Owner,
    });

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmCreated { name, description },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, name: String, description: String)]
pub struct UpdateRealm<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        realloc = crate::realm_space!(
            name,
            description,
            realm.masters.len()
        ),
        realloc::payer = master,
        realloc::zero = false,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() &&
            (m.role == RealmMasterRole::Owner || m.role == RealmMasterRole::Admin)
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_realm(
    ctx: Context<UpdateRealm>,
    _realm_id: String,
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
#[instruction(realm_id: String)]
pub struct DeleteRealm<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        close = master,
        constraint = realm.masters.iter().any(|candidate|
            candidate.pubkey == master.key() &&
            candidate.role == RealmMasterRole::Owner
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn delete_realm(ctx: Context<DeleteRealm>, _realm_id: String) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmDeleted {},
    });

    Ok(())
}
