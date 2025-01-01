use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(realm_id: String, location_id: String, name: String, tileset: String, tilemap: String)]
pub struct AddRealmLocation<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        realloc = crate::realm_space!(
            realm.name,
            realm.description,
            realm.masters.len(),
            realm.locations.len() + 1 // Increment
        ),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        init,
        payer = master,
        space = crate::realm_location_space!(name, tileset, tilemap),
        seeds = [LOCATION_SEED, realm_id.as_bytes(), location_id.as_bytes()],
        bump
    )]
    pub location: Account<'info, RealmLocation>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() &&
            (m.role == RealmMasterRole::Owner || m.role == RealmMasterRole::Admin)
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn add_realm_location(
    ctx: Context<AddRealmLocation>,
    _realm_id: String,
    _location_id: String,
    name: String,
    tileset: String,
    tilemap: String,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, ErrorCode::NameTooLong);
    require!(
        tileset.len() <= MAX_RESOURCE_PATH_LEN,
        ErrorCode::ResourcePathTooLong
    );
    require!(
        tilemap.len() <= MAX_RESOURCE_PATH_LEN,
        ErrorCode::ResourcePathTooLong
    );

    let location = &mut ctx.accounts.location;
    location.realm = ctx.accounts.realm.key();
    location.name = name.clone();
    location.tileset = tileset.clone();
    location.tilemap = tilemap.clone();

    ctx.accounts.realm.locations.push(location.key());

    emit!(LocationEvent {
        realm_pubkey: ctx.accounts.realm.key(),
        location_pubkey: location.key(),
        event_type: LocationEventType::LocationAdded {
            name,
            tilemap,
            tileset
        },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, location_id: String, name: String, tileset: String, tilemap: String)]
pub struct UpdateRealmLocation<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        seeds = [LOCATION_SEED, realm_id.as_bytes(), location_id.as_bytes()],
        bump,
        realloc = crate::realm_location_space!(name, tileset, tilemap),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub location: Account<'info, RealmLocation>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() &&
            (m.role == RealmMasterRole::Owner || m.role == RealmMasterRole::Admin)
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_realm_location(
    ctx: Context<UpdateRealmLocation>,
    _realm_id: String,
    _location_id: String,
    name: String,
    tileset: String,
    tilemap: String,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, ErrorCode::NameTooLong);
    require!(
        tileset.len() <= MAX_RESOURCE_PATH_LEN,
        ErrorCode::ResourcePathTooLong
    );
    require!(
        tilemap.len() <= MAX_RESOURCE_PATH_LEN,
        ErrorCode::ResourcePathTooLong
    );

    let location = &mut ctx.accounts.location;
    location.name = name.clone();
    location.tileset = tileset.clone();
    location.tilemap = tilemap.clone();

    emit!(LocationEvent {
        realm_pubkey: ctx.accounts.realm.key(),
        location_pubkey: location.key(),
        event_type: LocationEventType::LocationUpdated {
            name,
            tilemap,
            tileset
        },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, location_id: String)]
pub struct RemoveRealmLocation<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        realloc = crate::realm_space!(
            realm.name,
            realm.description,
            realm.masters.len(),
            realm.locations.len() - 1 // Decrement
        ),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        seeds = [LOCATION_SEED, realm_id.as_bytes(), location_id.as_bytes()],
        bump,
        close = master
    )]
    pub location: Account<'info, RealmLocation>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() &&
            (m.role == RealmMasterRole::Owner || m.role == RealmMasterRole::Admin)
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn remove_realm_location(
    ctx: Context<RemoveRealmLocation>,
    _realm_id: String,
    _location_id: String,
) -> Result<()> {
    ctx.accounts
        .realm
        .locations
        .retain(|l| l != &ctx.accounts.location.key());

    emit!(LocationEvent {
        realm_pubkey: ctx.accounts.realm.key(),
        location_pubkey: ctx.accounts.location.key(),
        event_type: LocationEventType::LocationRemoved {},
    });

    Ok(())
}
