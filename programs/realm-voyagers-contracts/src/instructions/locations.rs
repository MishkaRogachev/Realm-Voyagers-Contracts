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
            m.pubkey == master.key() && m.can_add_realm_location()
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

    let realm = &mut ctx.accounts.realm;
    let location = &mut ctx.accounts.location;

    location.realm = realm.key();
    location.owner = *ctx.accounts.master.key;

    location.name = name.clone();
    location.tileset = tileset.clone();
    location.tilemap = tilemap.clone();

    realm.locations.push(location.key());
    if realm.starting_location.is_none() {
        realm.starting_location = Some(location.key());
    }

    emit!(LocationEvent {
        realm_pubkey: realm.key(),
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
            m.pubkey == master.key() && m.can_manage_realm_location(&location)
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
            m.pubkey == master.key() && m.can_manage_realm_location(&location)
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
    let realm = &mut ctx.accounts.realm;

    realm
        .locations
        .retain(|l| l != &ctx.accounts.location.key());

    if realm.starting_location == Some(ctx.accounts.location.key()) {
        realm.starting_location = None;
        realm.starting_position = Position::default();
    }

    emit!(LocationEvent {
        realm_pubkey: ctx.accounts.realm.key(),
        location_pubkey: ctx.accounts.location.key(),
        event_type: LocationEventType::LocationRemoved {},
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, location_id: String)]
pub struct SetRealmStartingLocation<'info> {
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
    )]
    pub location: Account<'info, RealmLocation>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_set_realm_starting_location()
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn set_realm_starting_point(
    ctx: Context<SetRealmStartingLocation>,
    _realm_id: String,
    _location_id: String,
    position: Position,
) -> Result<()> {
    ctx.accounts.realm.starting_location = Some(ctx.accounts.location.key());
    ctx.accounts.realm.starting_position = position;

    Ok(())
}
