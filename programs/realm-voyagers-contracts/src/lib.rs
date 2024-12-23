use anchor_lang::prelude::*;

declare_id!("CaSHnhSk8WAV46aSkdAwZ1fqiqskTUQZDFatNFWJtxHT");

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod realm_voyagers {
    use super::*;

    pub fn create_realm(ctx: Context<CreateRealm>, name: String) -> Result<()> {
        realms::create_realm(ctx, name)
    }

    pub fn update_realm(ctx: Context<UpdateRealm>, name: String) -> Result<()> {
        realms::update_realm(ctx, name)
    }

    pub fn delete_realm(ctx: Context<DeleteRealm>) -> Result<()> {
        realms::delete_realm(ctx)
    }
}
