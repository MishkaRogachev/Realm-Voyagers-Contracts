use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct RealmDescription {
    pub name: String,
    pub details: String,
    pub logo: String,
}

#[account]
pub struct Realm {
    pub description: RealmDescription,

    pub created_at: i64,
    pub updated_at: i64,

    pub masters: Vec<crate::state::RealmMaster>,
    pub locations: Vec<Pubkey>,

    pub starting_location: Option<Pubkey>,
    pub starting_position: crate::state::Position,
}

#[macro_export]
macro_rules! realm_space {
    ($description:expr, $masters_count:expr, $locations_count:expr) => {
        // From https://book.anchor-lang.com/anchor_references/space.html
        8 +                                                                         // Discriminator
        4 + $description.name.len() +                                               // String prefix + content
        4 + $description.details.len() +                                            // String prefix + content
        4 + $description.logo.len() +                                               // String prefix + content
        8 +                                                                         // i64 timestamp
        8 +                                                                         // i64 timestamp
        4 + $masters_count * std::mem::size_of::<crate::state::RealmMaster>() +     // Vec prefix + content
        4 + $locations_count * std::mem::size_of::<Pubkey>() +                      // Vec prefix + content
        1 + std::mem::size_of::<Pubkey>() +                                         // Option + Pubkey
        std::mem::size_of::<crate::state::Position>()                               // Position
    };
}

impl RealmDescription {
    pub fn validate(&self) -> Result<()> {
        require!(self.name.len() <= MAX_NAME_LEN, ErrorCode::NameTooLong);
        require!(
            self.details.len() <= MAX_DETAILS_LEN,
            ErrorCode::DetailsTooLong
        );
        Ok(())
    }
}
