use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(realm_id: String, new_master_pubkey: Pubkey)]
pub struct AddRealmMaster<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        realloc = crate::realm_space!(
            realm.description,
            realm.masters.len() + 1, // Increment
            realm.dimensions.len()
        ),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_manage_realm_masters()
        ) @ ErrorCode::UnauthorizedRealmMaster,
        constraint = !realm.masters.iter().any(|m|
            m.pubkey == new_master_pubkey
        ) @ ErrorCode::DuplicateRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn add_realm_master(
    ctx: Context<AddRealmMaster>,
    _realm_id: String,
    new_master_pubkey: Pubkey,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    let master = RealmMaster {
        pubkey: new_master_pubkey,
        role: RealmMasterRole::Admin,
    };

    realm.masters.push(master.clone());
    realm.updated_at = Clock::get()?.unix_timestamp;

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmMasterAdded { master },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, master_pubkey: Pubkey)]
pub struct RemoveRealmMaster<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        realloc = crate::realm_space!(
            realm.description,
            realm.masters.len() - 1, // Decrement
            realm.dimensions.len()
        ),
        realloc::payer = master,
        realloc::zero = false,
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_manage_realm_masters()
        ) @ ErrorCode::UnauthorizedRealmMaster,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master_pubkey
        ) @ ErrorCode::RealmMasterNotFound,
        constraint = !realm.masters.contains(&RealmMaster {
            pubkey: master_pubkey,
            role: RealmMasterRole::Owner
        }) @ ErrorCode::CantRemoveRealmOwner
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn remove_realm_master(
    ctx: Context<RemoveRealmMaster>,
    _realm_id: String,
    master_pubkey: Pubkey,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    let master_index = realm
        .masters
        .iter()
        .position(|master| master.pubkey == master_pubkey)
        .ok_or(ErrorCode::RealmMasterNotFound)?;

    let master = realm.masters.swap_remove(master_index);
    realm.updated_at = Clock::get()?.unix_timestamp;

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmMasterRemoved { master },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String)]
pub struct TransferRealmOwnership<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_manage_realm_masters()
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn transfer_realm_ownership(
    ctx: Context<TransferRealmOwnership>,
    _realm_id: String,
    new_owner_pubkey: Pubkey,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    let old_owner_index = realm
        .masters
        .iter()
        .position(|master| master.pubkey == *ctx.accounts.master.key)
        .ok_or(ErrorCode::UnauthorizedRealmMaster)?;

    let new_owner_index = realm
        .masters
        .iter()
        .position(|master| master.pubkey == new_owner_pubkey)
        .ok_or(ErrorCode::RealmMasterNotFound)?;

    // Current roles should be checked via constraints
    realm.masters[old_owner_index].role = RealmMasterRole::Admin;
    realm.masters[new_owner_index].role = RealmMasterRole::Owner;
    realm.updated_at = Clock::get()?.unix_timestamp;

    let old_owner = realm.masters[old_owner_index].clone();
    let new_owner = realm.masters[new_owner_index].clone();

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmOwnershipTransferred {
            old_owner: old_owner,
            new_owner: new_owner,
        },
    });

    Ok(())
}
