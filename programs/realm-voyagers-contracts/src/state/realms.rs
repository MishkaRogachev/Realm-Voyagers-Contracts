use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Realm {
    pub realm_master: Pubkey,

    #[max_len(MAX_SEED_LEN)]
    pub seed: String,

    #[max_len(MAX_NAME_LEN)]
    pub name: String,

    #[max_len(MAX_DESCRIPTION_LEN)]
    pub description: String,

    pub created_at: i64,
}
