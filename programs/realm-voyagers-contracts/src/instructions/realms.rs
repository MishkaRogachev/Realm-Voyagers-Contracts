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
        seeds = [REALM_SEED, id.as_bytes()],
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
#[instruction(id: String)]
pub struct UpdateRealm<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, id.as_bytes()],
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

    require!(
        has_role(
            realm,
            ctx.accounts.master.key(),
            vec![RealmMasterRole::Owner, RealmMasterRole::Admin]
        ),
        ErrorCode::UnauthorizedRealmMaster
    );

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
        seeds = [REALM_SEED, id.as_bytes()],
        bump,
        close = master,
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn delete_realm(ctx: Context<DeleteRealm>, _id: String) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    require!(
        has_role(
            realm,
            ctx.accounts.master.key(),
            vec![RealmMasterRole::Owner]
        ),
        ErrorCode::UnauthorizedRealmMaster
    );

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmDeleted {},
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct AddRealmMaster<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, id.as_bytes()],
        bump,
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn add_realm_master(
    ctx: Context<AddRealmMaster>,
    _id: String,
    new_master_pubkey: Pubkey,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    require!(
        has_role(
            realm,
            ctx.accounts.master.key(),
            vec![RealmMasterRole::Owner]
        ),
        ErrorCode::UnauthorizedRealmMaster
    );

    realm.masters.push(RealmMaster {
        pubkey: new_master_pubkey,
        role: RealmMasterRole::Admin,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct RemoveRealmMaster<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, id.as_bytes()],
        bump,
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn remove_realm_master(
    ctx: Context<RemoveRealmMaster>,
    _id: String,
    master_pubkey: Pubkey,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    require!(
        has_role(
            realm,
            ctx.accounts.master.key(),
            vec![RealmMasterRole::Owner]
        ),
        ErrorCode::UnauthorizedRealmMaster
    );

    require!(
        realm
            .masters
            .iter()
            .any(|master| master.pubkey == master_pubkey),
        ErrorCode::RealmMasterNotFound
    );

    require!(
        !realm.masters.contains(&RealmMaster {
            pubkey: master_pubkey,
            role: RealmMasterRole::Owner
        }),
        ErrorCode::CantRemoveRealmOwner
    );

    realm
        .masters
        .retain(|master| master.pubkey != master_pubkey);

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct TransferRealmOwnership<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, id.as_bytes()],
        bump,
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn transfer_realm_ownership(
    ctx: Context<TransferRealmOwnership>,
    _id: String,
    new_owner_pubkey: Pubkey,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    require!(
        has_role(
            realm,
            ctx.accounts.master.key(),
            vec![RealmMasterRole::Owner]
        ),
        ErrorCode::UnauthorizedRealmMaster
    );

    require!(
        realm
            .masters
            .iter()
            .any(|master| master.pubkey == new_owner_pubkey),
        ErrorCode::RealmMasterNotFound
    );

    realm.masters.iter_mut().for_each(|master| {
        if master.pubkey == new_owner_pubkey {
            master.role = RealmMasterRole::Owner;
        } else if master.pubkey == *ctx.accounts.master.key {
            master.role = RealmMasterRole::Admin;
        }
    });

    Ok(())
}

fn has_role(realm: &Realm, pubkey: Pubkey, roles: Vec<RealmMasterRole>) -> bool {
    realm
        .masters
        .iter()
        .any(|master| master.pubkey == pubkey && roles.contains(&master.role))
}
