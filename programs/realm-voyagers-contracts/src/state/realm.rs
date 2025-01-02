use anchor_lang::prelude::*;

#[account]
pub struct Realm {
    pub name: String,
    pub description: String,
    pub created_at: i64,

    pub masters: Vec<crate::state::RealmMaster>,
    pub locations: Vec<Pubkey>,

    pub starting_location: Option<Pubkey>,
    pub starting_position: crate::state::Position,
}

#[macro_export]
macro_rules! realm_space {
    ($name:expr, $description:expr, $masters_count:expr, $locations_count:expr) => {
        // From https://book.anchor-lang.com/anchor_references/space.html
        8 +                                                         // discriminator
        4 + $name.len() +                                           // String prefix + content
        4 + $description.len() +                                    // String prefix + content
        8 +                                                         // i64 timestamp
        4 + $masters_count * std::mem::size_of::<crate::state::RealmMaster>() +   // Vec prefix + content
        4 + $locations_count * std::mem::size_of::<Pubkey>() +      // Vec prefix + content
        1 + std::mem::size_of::<Pubkey>() +                         // Option + Pubkey
        std::mem::size_of::<crate::state::Position>()               // Position
    };
}
