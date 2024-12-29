use anchor_lang::prelude::*;

declare_id!("CaSHnhSk8WAV46aSkdAwZ1fqiqskTUQZDFatNFWJtxHT");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod realm_voyagers {
    use super::*;

    pub fn create_realm(
        ctx: Context<CreateRealm>,
        id: String,
        name: String,
        description: String,
    ) -> Result<()> {
        realms::create_realm(ctx, id, name, description)
    }

    pub fn update_realm(
        ctx: Context<UpdateRealm>,
        id: String,
        name: String,
        description: String,
    ) -> Result<()> {
        realms::update_realm(ctx, id, name, description)
    }

    pub fn delete_realm(ctx: Context<DeleteRealm>, id: String) -> Result<()> {
        realms::delete_realm(ctx, id)
    }
}
