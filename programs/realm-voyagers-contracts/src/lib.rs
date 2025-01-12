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

    // Realm Dimensions
    pub fn add_realm_dimension(
        ctx: Context<AddRealmDimension>,
        realm_id: String,
        dimension_id: String,
        name: String,
        areas: Vec<state::RealmDimensionArea>,
    ) -> Result<()> {
        dimensions::add_realm_dimension(ctx, realm_id, dimension_id, name, areas)
    }

    pub fn update_realm_dimension(
        ctx: Context<UpdateRealmDimension>,
        realm_id: String,
        dimension_id: String,
        name: String,
        areas: Vec<state::RealmDimensionArea>,
    ) -> Result<()> {
        dimensions::update_realm_dimension(ctx, realm_id, dimension_id, name, areas)
    }

    pub fn remove_realm_dimension(
        ctx: Context<RemoveRealmDimension>,
        realm_id: String,
        dimension_id: String,
    ) -> Result<()> {
        dimensions::remove_realm_dimension(ctx, realm_id, dimension_id)
    }

    pub fn set_realm_starting_point(
        ctx: Context<SetRealmStartingDimension>,
        realm_id: String,
        dimension_id: String,
        position: state::Position,
    ) -> Result<()> {
        dimensions::set_realm_starting_point(ctx, realm_id, dimension_id, position)
    }

    // Heroes
    pub fn create_hero(
        ctx: Context<CreateHero>,
        hero_id: String,
        description: state::HeroDescription,
    ) -> Result<()> {
        heroes::create_hero(ctx, hero_id, description)
    }

    pub fn update_hero_description(
        ctx: Context<UpdateHeroDescription>,
        hero_id: String,
        description: state::HeroDescription,
    ) -> Result<()> {
        heroes::update_hero_description(ctx, hero_id, description)
    }

    // Journey
    pub fn start_journey(ctx: Context<StartJourney>, realm_id: String) -> Result<()> {
        journeys::start_journey(ctx, realm_id)
    }
}
