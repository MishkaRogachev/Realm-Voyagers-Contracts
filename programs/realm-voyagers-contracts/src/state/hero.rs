use anchor_lang::prelude::*;

use crate::state::hero_stats::*;

#[account]
pub struct Hero {
    pub master: Pubkey,        // Owner of the hero class
    pub name: String,          // Base name of the hero class
    pub graphics: String,      // URL or IPFS hash for the hero class graphics
    pub base_stats: HeroStats, // Base stats for the hero class
    pub lore: String,          // Background of the hero

    pub created_at: i64,
    pub updated_at: i64,
}

#[account]
pub struct PlayerHero {
    pub player: Pubkey, // Owner of the hero instance
    pub hero: Pubkey,   // Reference to the Hero account
    pub name: String,   // Overrided name of the hero

    pub level: u64, // Level
    pub xp: u64,    // Experience points
    pub stats: HeroStats,
    pub items: Vec<Pubkey>, // Inventory of items (stored as item PDAs or unique IDs)

    pub created_at: i64,
    pub updated_at: i64,
}

#[macro_export]
macro_rules! hero_space {
    ($name:expr, $graphics:expr, $lore:expr) => {
        8 +                                 // discriminator
        32 +                                // master: pubkey
        4 + $name.len() +                   // name: String prefix + content
        4 + $graphics.len() +               // graphics: String prefix + content
        std::mem::size_of::<HeroStats>() +  // base_stats: HeroStats
        4 + $lore.len() +                   // lore: String prefix + content
        8 +                                 // created_at: i64
        8                                   // updated_at: i64
    };
}

#[macro_export]
macro_rules! player_hero_space {
    ($name:expr, $items_count:expr) => {
        8 +                                 // discriminator
        32 +                                // player: pubkey
        32 +                                // hero: pubkey
        4 + $name.len() +                   // name: String prefix + content
        8 +                                 // level: u64
        8 +                                 // xp: u64
        std::mem::size_of::<HeroStats>() +  // stats: HeroStats (8 stats of u64)
        4 + $items_count * 32 +             // items: Vec<Pubkey> (4 bytes prefix + items as pubkeys)
        8 +                                 // created_at: i64
        8                                   // updated_at: i64
    };
}
