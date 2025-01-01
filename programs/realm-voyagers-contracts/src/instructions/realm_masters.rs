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
            realm.name,
            realm.description,
            realm.masters.len() + 1, // Increment
            realm.locations.len()
        ),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() &&
            m.role == RealmMasterRole::Owner
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
            realm.name,
            realm.description,
            realm.masters.len() - 1, // Decrement
            realm.locations.len()
        ),
        realloc::payer = master,
        realloc::zero = false,
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() &&
            m.role == RealmMasterRole::Owner
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
        constraint = realm.masters.iter().any(|candidate|
            candidate.pubkey == master.key() &&
            candidate.role == RealmMasterRole::Owner
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

    let old_owner = realm
        .masters
        .iter()
        .find(|master| master.pubkey == *ctx.accounts.master.key)
        .cloned();

    let new_owner = realm
        .masters
        .iter()
        .find(|master| master.pubkey == new_owner_pubkey)
        .cloned();

    require!(old_owner.is_some(), ErrorCode::UnauthorizedRealmMaster); // Double check, should never happen
    require!(new_owner.is_some(), ErrorCode::RealmMasterNotFound);

    realm.masters.iter_mut().for_each(|master| {
        if master.pubkey == new_owner_pubkey {
            master.role = RealmMasterRole::Owner;
        } else if master.pubkey == *ctx.accounts.master.key {
            master.role = RealmMasterRole::Admin;
        }
    });

    // Emit event
    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmOwnershipTransferred {
            old_owner: old_owner.unwrap(),
            new_owner: new_owner.unwrap(),
        },
    });

    Ok(())
}
