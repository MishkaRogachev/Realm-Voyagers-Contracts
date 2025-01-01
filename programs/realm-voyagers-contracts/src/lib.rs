use anchor_lang::prelude::*;

declare_id!("CaSHnhSk8WAV46aSkdAwZ1fqiqskTUQZDFatNFWJtxHT");

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod realm_voyagers {
    use super::*;

    // Realms
    pub fn create_realm(
        ctx: Context<CreateRealm>,
        realm_id: String,
        name: String,
        description: String,
    ) -> Result<()> {
        realms::create_realm(ctx, realm_id, name, description)
    }

    pub fn update_realm(
        ctx: Context<UpdateRealm>,
        realm_id: String,
        name: String,
        description: String,
    ) -> Result<()> {
        realms::update_realm(ctx, realm_id, name, description)
    }

    pub fn delete_realm(ctx: Context<DeleteRealm>, realm_id: String) -> Result<()> {
        realms::delete_realm(ctx, realm_id)
    }

    // Realm Masters
    pub fn add_realm_master(
        ctx: Context<AddRealmMaster>,
        realm_id: String,
        new_master_pubkey: Pubkey,
    ) -> Result<()> {
        realm_masters::add_realm_master(ctx, realm_id, new_master_pubkey)
    }

    pub fn remove_realm_master(
        ctx: Context<RemoveRealmMaster>,
        realm_id: String,
        master_pubkey: Pubkey,
    ) -> Result<()> {
        realm_masters::remove_realm_master(ctx, realm_id, master_pubkey)
    }

    pub fn transfer_realm_ownership(
        ctx: Context<TransferRealmOwnership>,
        realm_id: String,
        new_owner_pubkey: Pubkey,
    ) -> Result<()> {
        realm_masters::transfer_realm_ownership(ctx, realm_id, new_owner_pubkey)
    }

    // Realm Locations
    pub fn add_realm_location(
        ctx: Context<AddRealmLocation>,
        realm_id: String,
        location_id: String,
        name: String,
        tileset: String,
        tilemap: String,
    ) -> Result<()> {
        realm_locations::add_realm_location(ctx, realm_id, location_id, name, tileset, tilemap)
    }

    pub fn update_realm_location(
        ctx: Context<UpdateRealmLocation>,
        realm_id: String,
        location_id: String,
        name: String,
        tileset: String,
        tilemap: String,
    ) -> Result<()> {
        realm_locations::update_realm_location(ctx, realm_id, location_id, name, tileset, tilemap)
    }

    pub fn remove_realm_location(
        ctx: Context<RemoveRealmLocation>,
        realm_id: String,
        location_id: String,
    ) -> Result<()> {
        realm_locations::remove_realm_location(ctx, realm_id, location_id)
    }
}
