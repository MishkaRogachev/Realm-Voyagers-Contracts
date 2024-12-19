pub mod errors;
pub mod realms;

use crate::realms::*;
use anchor_lang::prelude::*;

declare_id!("3sEiguGdXKZ3xmQYdr2L12k2QAB7ZjUX5kgRfX4oARby");

#[program]
pub mod realm_voyagers_contracts {
    use super::*;

    pub fn create_realm(ctx: Context<CreateRealm>, name: String) -> Result<()> {
        let realm = &mut ctx.accounts.realm;
        realm.master = ctx.accounts.master.key();
        realm.name = name;
        realm.created_at = Clock::get()?.unix_timestamp;
        msg!("Realm '{}' created by {}", realm.name, realm.master);
        Ok(())
    }

    pub fn update_realm(ctx: Context<UpdateRealm>, name: String) -> Result<()> {
        let realm = &mut ctx.accounts.realm;
        realm.name = name;
        msg!("Realm updated to '{}'", realm.name);
        Ok(())
    }

    pub fn add_realm_location(
        ctx: Context<AddRealmLocation>,
        name: String,
        tilemap_url: String,
        tileset_url: String,
    ) -> Result<()> {
        let location = &mut ctx.accounts.location;
        location.realm = ctx.accounts.realm.key();
        location.name = name;
        location.tilemap_url = tilemap_url;
        location.tileset_url = tileset_url;
        msg!(
            "Location '{}' added to Realm '{}'",
            location.name,
            ctx.accounts.realm.name
        );
        Ok(())
    }

    pub fn update_realm_location(
        ctx: Context<UpdateRealmLocation>,
        name: String,
        tilemap_url: String,
        tileset_url: String,
    ) -> Result<()> {
        let location = &mut ctx.accounts.location;
        location.name = name;
        location.tilemap_url = tilemap_url;
        location.tileset_url = tileset_url;
        msg!("Location '{}' updated", location.name);
        Ok(())
    }

    pub fn remove_realm_location(ctx: Context<RemoveRealmLocation>) -> Result<()> {
        msg!(
            "Location '{}' removed from Realm '{}'",
            ctx.accounts.location.name,
            ctx.accounts.realm.name
        );
        Ok(())
    }

    pub fn delete_realm(ctx: Context<DeleteRealm>) -> Result<()> {
        msg!("Realm '{}' deleted", ctx.accounts.realm.name);
        Ok(())
    }
}
