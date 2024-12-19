use super::errors::ErrorCode;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Location {
    #[max_len(64)]
    pub name: String,
    #[max_len(72)] // 64 for CID and 8 for prefix
    pub map_uri: String,
}

#[account]
#[derive(InitSpace)]
pub struct Realm {
    pub master: Pubkey,
    #[max_len(64)]
    pub name: String,
    pub created_at: i64,
    #[max_len(16)]
    pub locations: Vec<Location>,
    pub starting_location_index: u8,
}

impl Realm {
    pub fn validate(&self) -> Result<()> {
        if self.locations.is_empty() {
            return Err(ErrorCode::NoLocationsProvieded.into());
        }
        if self.starting_location_index as usize >= self.locations.len() {
            return Err(ErrorCode::StartingLocationOutOfRange.into());
        }
        Ok(())
    }
}
