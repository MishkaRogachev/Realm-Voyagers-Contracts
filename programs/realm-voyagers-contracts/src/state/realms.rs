use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Realm {
    #[max_len(MAX_NAME_LEN)]
    pub name: String,

    #[max_len(MAX_DESCRIPTION_LEN)]
    pub description: String,

    pub created_at: i64,
}
