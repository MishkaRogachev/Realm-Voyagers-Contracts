use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct CreateRealm<'info> {
    #[account(
        init,
        payer = realm_master,
        space = 8 + Realm::INIT_SPACE,
        seeds = [realm_master.key().as_ref()],
        bump
    )]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub realm_master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_realm(ctx: Context<CreateRealm>, name: String) -> Result<()> {
    let realm = &mut ctx.accounts.realm;
    realm.realm_master = ctx.accounts.realm_master.key();
    realm.created_at = Clock::get()?.unix_timestamp;
    realm.name = name;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateRealm<'info> {
    #[account(mut, has_one = realm_master)]
    pub realm: Account<'info, Realm>,
    pub realm_master: Signer<'info>,
}

pub fn update_realm(ctx: Context<UpdateRealm>, name: String) -> Result<()> {
    let realm = &mut ctx.accounts.realm;
    realm.name = name;
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteRealm<'info> {
    #[account(mut, close = realm_master, has_one = realm_master)]
    pub realm: Account<'info, Realm>,
    pub realm_master: Signer<'info>,
}

pub fn delete_realm(_ctx: Context<DeleteRealm>) -> Result<()> {
    Ok(())
}
