use anchor_lang::prelude::*;

use crate::state::*;

const REALM_SEED: &[u8] = b"realm";

#[event]
pub struct RealmCreated {
    pub realm_pubkey: Pubkey,
    pub name: String,
    pub description: String,
}

#[event]
pub struct RealmUpdated {
    pub realm_pubkey: Pubkey,
    pub name: String,
    pub description: String,
}

#[event]
pub struct RealmDeleted {
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
    realm.name = name;
    realm.description = description;
    realm.created_at = Clock::get()?.unix_timestamp;

    emit!(RealmCreated {
        realm_pubkey: realm.key(),
        name: realm.name.clone(),
        description: realm.description.clone(),
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
    realm.name = name;
    realm.description = description;

    emit!(RealmUpdated {
        realm_pubkey: realm.key(),
        name: realm.name.clone(),
        description: realm.description.clone(),
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
    emit!(RealmDeleted {
        realm_pubkey: ctx.accounts.realm.key(),
    });

    Ok(())
}
