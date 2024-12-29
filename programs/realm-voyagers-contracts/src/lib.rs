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

    pub fn add_realm_master(
        ctx: Context<AddRealmMaster>,
        id: String,
        new_master_pubkey: Pubkey,
    ) -> Result<()> {
        realms::add_realm_master(ctx, id, new_master_pubkey)
    }

    pub fn remove_realm_master(
        ctx: Context<RemoveRealmMaster>,
        id: String,
        master_pubkey: Pubkey,
    ) -> Result<()> {
        realms::remove_realm_master(ctx, id, master_pubkey)
    }

    pub fn transfer_realm_ownership(
        ctx: Context<TransferRealmOwnership>,
        id: String,
        new_owner_pubkey: Pubkey,
    ) -> Result<()> {
        realms::transfer_realm_ownership(ctx, id, new_owner_pubkey)
    }
}
