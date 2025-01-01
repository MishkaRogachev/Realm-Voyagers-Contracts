use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum RealmMasterRole {
    Owner = 0,
    Admin = 1,
    Curator = 2,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct RealmMaster {
    pub pubkey: Pubkey,
    pub role: RealmMasterRole,
}

#[account]
pub struct Realm {
    pub name: String,
    pub description: String,
    pub created_at: i64,

    pub masters: Vec<RealmMaster>,
    pub locations: Vec<Pubkey>,
}

#[macro_export]
macro_rules! realm_space {
    ($name:expr, $description:expr, $masters_count:expr, $locations_count:expr) => {
        // From https://book.anchor-lang.com/anchor_references/space.html
        8 +                                                         // discriminator
        4 + $name.len() +                                           // String prefix + content
        4 + $description.len() +                                    // String prefix + content
        8 +                                                         // i64 timestamp
        4 + $masters_count * std::mem::size_of::<RealmMaster>() +   // Vec prefix + content
        4 + $locations_count * std::mem::size_of::<Pubkey>()        // Vec prefix + content
    };
}
