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
        description: state::RealmDescription,
    ) -> Result<()> {
        realms::create_realm(ctx, realm_id, description)
    }

    pub fn update_realm_description(
        ctx: Context<UpdateRealmDescription>,
        realm_id: String,
        description: state::RealmDescription,
    ) -> Result<()> {
        realms::update_realm_description(ctx, realm_id, description)
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
        masters::add_realm_master(ctx, realm_id, new_master_pubkey)
    }

    pub fn remove_realm_master(
        ctx: Context<RemoveRealmMaster>,
        realm_id: String,
        master_pubkey: Pubkey,
    ) -> Result<()> {
        masters::remove_realm_master(ctx, realm_id, master_pubkey)
    }

    pub fn transfer_realm_ownership(
        ctx: Context<TransferRealmOwnership>,
        realm_id: String,
        new_owner_pubkey: Pubkey,
    ) -> Result<()> {
        masters::transfer_realm_ownership(ctx, realm_id, new_owner_pubkey)
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
        locations::add_realm_location(ctx, realm_id, location_id, name, tileset, tilemap)
    }

    pub fn update_realm_location(
        ctx: Context<UpdateRealmLocation>,
        realm_id: String,
        location_id: String,
        name: String,
        tileset: String,
        tilemap: String,
    ) -> Result<()> {
        locations::update_realm_location(ctx, realm_id, location_id, name, tileset, tilemap)
    }

    pub fn remove_realm_location(
        ctx: Context<RemoveRealmLocation>,
        realm_id: String,
        location_id: String,
    ) -> Result<()> {
        locations::remove_realm_location(ctx, realm_id, location_id)
    }

    pub fn set_realm_starting_point(
        ctx: Context<SetRealmStartingLocation>,
        realm_id: String,
        location_id: String,
        position: state::Position,
    ) -> Result<()> {
        locations::set_realm_starting_point(ctx, realm_id, location_id, position)
    }

    // Journey
    pub fn start_journey(ctx: Context<StartJourney>, realm_id: String) -> Result<()> {
        journeys::start_journey(ctx, realm_id)
    }
}
