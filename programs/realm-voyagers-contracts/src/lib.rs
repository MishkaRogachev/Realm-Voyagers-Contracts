pub mod errors;
pub mod realms;

use anchor_lang::prelude::*;

const ANCHOR_DESCRIMINATOR_SIZE: usize = 8;
const REALM_SEED: &[u8] = b"realm";

declare_id!("3sEiguGdXKZ3xmQYdr2L12k2QAB7ZjUX5kgRfX4oARby");

#[program]
pub mod realm_voyagers_contracts {
    use super::*;

    pub fn create_realm(ctx: Context<CreateRealm>, name: String) -> Result<()> {
        let realm = &mut ctx.accounts.realm;
        realm.master = ctx.accounts.master.key();
        realm.name = name;
        realm.created_at = Clock::get()?.unix_timestamp;
        msg!("Realm {} created!", ctx.accounts.realm.name);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRealm<'info> {
    #[account(mut)]
    pub master: Signer<'info>,

    #[account(
        init,
        payer = master,
        space = ANCHOR_DESCRIMINATOR_SIZE + realms::Realm::INIT_SPACE,
        seeds = [REALM_SEED, master.key().as_ref()],
        bump
    )]
    pub realm: Account<'info, realms::Realm>,

    pub system_program: Program<'info, System>,
}
