use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Realm {
    pub realm_master: Pubkey,

    #[max_len(32)]
    pub seed: String,

    #[max_len(32)]
    pub name: String,

    #[max_len(128)]
    pub description: String,

    pub created_at: i64,
}
