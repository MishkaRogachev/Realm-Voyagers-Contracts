use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(realm_id: String, dimension_id: String, name: String)]
pub struct AddRealmDimension<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        realloc = crate::realm_space!(
            realm.description,
            realm.masters.len(),
            realm.dimensions.len() + 1 // Increment
        ),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        init,
        payer = master,
        space = crate::realm_dimension_space!(name, vec![]),
        seeds = [DIMENSION_SEED, realm_id.as_bytes(), dimension_id.as_bytes()],
        bump
    )]
    pub dimension: Account<'info, RealmDimension>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_add_realm_dimension()
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn add_realm_dimension(
    ctx: Context<AddRealmDimension>,
    _realm_id: String,
    _dimension_id: String,
    name: String,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, ErrorCode::NameTooLong);

    let realm = &mut ctx.accounts.realm;
    let dimension = &mut ctx.accounts.dimension;

    dimension.realm = realm.key();
    dimension.owner = *ctx.accounts.master.key;

    dimension.name = name.clone();

    realm.dimensions.push(dimension.key());
    if realm.starting_dimension.is_none() {
        realm.starting_dimension = Some(dimension.key());
    }
    realm.updated_at = Clock::get()?.unix_timestamp;

    emit!(RealmDimensionEvent {
        realm_pubkey: realm.key(),
        dimension_pubkey: dimension.key(),
        event_type: RealmDimensionEventType::DimensionAdded { name },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, dimension_id: String, name: String)]
pub struct RenameRealmDimension<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        seeds = [DIMENSION_SEED, realm_id.as_bytes(), dimension_id.as_bytes()],
        bump,
        realloc = crate::realm_dimension_space!(name, vec![]),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub dimension: Account<'info, RealmDimension>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_manage_realm_dimension(&dimension)
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn rename_realm_dimension(
    ctx: Context<RenameRealmDimension>,
    _realm_id: String,
    _dimension_id: String,
    name: String,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, ErrorCode::NameTooLong);

    let dimension = &mut ctx.accounts.dimension;
    dimension.name = name.clone();

    emit!(RealmDimensionEvent {
        realm_pubkey: ctx.accounts.realm.key(),
        dimension_pubkey: dimension.key(),
        event_type: RealmDimensionEventType::DimensionRenamed { name },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, dimension_id: String)]
pub struct RemoveRealmDimension<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        realloc = crate::realm_space!(
            realm.description,
            realm.masters.len(),
            realm.dimensions.len() - 1 // Decrement
        ),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        seeds = [DIMENSION_SEED, realm_id.as_bytes(), dimension_id.as_bytes()],
        bump,
        close = master
    )]
    pub dimension: Account<'info, RealmDimension>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_manage_realm_dimension(&dimension)
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn remove_realm_dimension(
    ctx: Context<RemoveRealmDimension>,
    _realm_id: String,
    _dimension_id: String,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    realm
        .dimensions
        .retain(|l| l != &ctx.accounts.dimension.key());

    if realm.starting_dimension == Some(ctx.accounts.dimension.key()) {
        realm.starting_dimension = None;
        realm.starting_position = Position::default();
    }
    realm.updated_at = Clock::get()?.unix_timestamp;

    emit!(RealmDimensionEvent {
        realm_pubkey: ctx.accounts.realm.key(),
        dimension_pubkey: ctx.accounts.dimension.key(),
        event_type: RealmDimensionEventType::DimensionRemoved {},
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, dimension_id: String)]
pub struct SetRealmStartingDimension<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        seeds = [DIMENSION_SEED, realm_id.as_bytes(), dimension_id.as_bytes()],
        bump,
    )]
    pub dimension: Account<'info, RealmDimension>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_set_realm_starting_point()
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn set_realm_starting_point(
    ctx: Context<SetRealmStartingDimension>,
    _realm_id: String,
    _dimension_id: String,
    position: Position,
) -> Result<()> {
    let realm = &mut ctx.accounts.realm;
    realm.starting_dimension = Some(ctx.accounts.dimension.key());
    realm.starting_position = position;
    realm.updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}
