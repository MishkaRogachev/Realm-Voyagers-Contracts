use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct RealmDimensionArea {
    pub name: String,

    pub area: crate::state::Rect,

    pub tileset: String,
    pub tilemap: String,
}

#[account]
pub struct RealmDimension {
    pub realm: Pubkey,
    pub owner: Pubkey,

    pub name: String,

    pub areas: Vec<RealmDimensionArea>,
}

#[macro_export]
macro_rules! realm_dimension_space {
    ($name:expr, $areas:expr) => {{
        // From https://book.anchor-lang.com/anchor_references/space.html
        let mut total_area_size = 0;
        for area in $areas {
            let area: &RealmDimensionArea = &area;
            total_area_size += 4 + area.name.len();                         // name: String (4 bytes prefix + content)
            total_area_size += std::mem::size_of::<crate::state::Rect>();   // area: Rect (fixed size)
            total_area_size += 4 + area.tileset.len();                      // tileset: String (4 bytes prefix + content)
            total_area_size += 4 + area.tilemap.len();                      // tilemap: String (4 bytes prefix + content)
        }

        8 +                                                                 // discriminator
        32 +                                                                // realm pubkey
        32 +                                                                // owner pubkey
        4 + $name.len() +                                                   // name: String (4 bytes prefix + content)
        4 + total_area_size                                                 // areas: Vec<RealmDimensionArea> (4 bytes prefix + content)
    }};
}
