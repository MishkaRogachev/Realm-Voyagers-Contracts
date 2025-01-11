use anchor_lang::prelude::*;

use crate::state::hero_tags_and_stats::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct HeroDescription {
    pub name: String,
    pub details: String,
    pub logo: String,
}

#[account]
pub struct Hero {
    pub owner: Pubkey, // Owner of the hero

    pub description: HeroDescription,

    pub tags: Vec<HeroTag>, // Tags/classifications for the hero
    pub level: u64,         // Level
    pub xp: u64,            // Experience points
    pub stats: HeroStats,
    pub items: Vec<Pubkey>, // Inventory of items (stored as item PDAs or unique IDs)

    pub created_at: i64,
    pub updated_at: i64,
}

#[macro_export]
macro_rules! hero_space {
    ($description:expr, $tags_count:expr, $items_count:expr) => {
        8 +                                 // discriminator
        32 +                                // owner: pubkey
        4 + $description.name.len() +       // String prefix + content
        4 + $description.details.len() +    // String prefix + content
        4 + $description.logo.len() +       // String prefix + content
        4 + $tags_count * 1 +               // tags: Vec<HeroTag> (4 bytes prefix + each tag is 1 byte)
        8 +                                 // level: u64
        8 +                                 // xp: u64
        (8 * 8) +                           // stats: HeroStats (8 stats of u64)
        4 + $items_count * 32 +             // items: Vec<Pubkey> (4 bytes prefix + items as pubkeys)
        8 +                                 // created_at: i64
        8                                   // updated_at: i64
    };
}
