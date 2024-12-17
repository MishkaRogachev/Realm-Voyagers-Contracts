use anchor_lang::prelude::*;

declare_id!("3sEiguGdXKZ3xmQYdr2L12k2QAB7ZjUX5kgRfX4oARby");

#[program]
pub mod realm_voyagers_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
