use anchor_lang::prelude::*;

#[account]
pub struct RealmDimension {
    pub realm: Pubkey,
    pub owner: Pubkey,

    pub name: String,
    pub tileset: String,
    pub tilemap: String,
}

#[macro_export]
macro_rules! realm_dimension_space {
    ($name:expr, $tileset:expr, $tilemap:expr) => {
        8 +                     // discriminator
        32 +                    // realm pubkey
        32 +                    // owner pubkey
        4 + $name.len() +       // String prefix + content
        4 + $tileset.len() +    // String prefix + content
        4 + $tilemap.len()      // String prefix + content
    };
}
