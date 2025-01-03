use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(realm_id: String, description: RealmDescription)]
pub struct CreateRealm<'info> {
    #[account(
        init,
        payer = master,
        space = crate::realm_space!(description, 1, 0),
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
    description: RealmDescription,
) -> Result<()> {
    description.validate()?;

    let realm = &mut ctx.accounts.realm;
    realm.description = description.clone();
    realm.created_at = Clock::get()?.unix_timestamp;
    realm.updated_at = realm.created_at;
    realm.masters.push(RealmMaster {
        pubkey: *ctx.accounts.master.key,
        role: RealmMasterRole::Owner,
    });

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmCreated { description },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(realm_id: String, description: RealmDescription)]
pub struct UpdateRealmDescription<'info> {
    #[account(
        mut,
        seeds = [REALM_SEED, realm_id.as_bytes()],
        bump,
        realloc = crate::realm_space!(
            description,
            realm.masters.len(),
            realm.dimensions.len()
        ),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_update_realm()
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_realm_description(
    ctx: Context<UpdateRealmDescription>,
    _realm_id: String,
    description: RealmDescription,
) -> Result<()> {
    description.validate()?;

    let realm = &mut ctx.accounts.realm;
    realm.description = description.clone();
    realm.updated_at = Clock::get()?.unix_timestamp;

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmDescriptionUpdated { description },
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
        close = master
    )]
    pub realm: Account<'info, Realm>,

    #[account(
        mut,
        constraint = realm.masters.iter().any(|m|
            m.pubkey == master.key() && m.can_delete_realm()
        ) @ ErrorCode::UnauthorizedRealmMaster
    )]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn delete_realm(ctx: Context<DeleteRealm>, _realm_id: String) -> Result<()> {
    let realm = &mut ctx.accounts.realm;

    // Iterate over and close all dimensions linked to the realm
    for dimension_pubkey in realm.dimensions.iter() {
        let dimension_info = ctx
            .remaining_accounts
            .iter()
            .find(|account_info| account_info.key == dimension_pubkey)
            .ok_or(ErrorCode::DimensionNotProvided)?;

        // TODO: transfer to dimension owner

        **ctx.accounts.master.lamports.borrow_mut() += dimension_info.lamports();
        **dimension_info.lamports.borrow_mut() = 0;
        dimension_info.try_borrow_mut_data()?.fill(0);
    }

    emit!(RealmEvent {
        realm_pubkey: realm.key(),
        event_type: RealmEventType::RealmDeleted {},
    });

    Ok(())
}
