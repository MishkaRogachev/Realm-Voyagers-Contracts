use crate::constants::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum RealmMasterRole {
    Owner = 0,
    Admin = 1,
    Curator = 2,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub struct RealmMaster {
    pub pubkey: Pubkey,
    pub role: RealmMasterRole,
}

#[account]
#[derive(InitSpace)]
pub struct Realm {
    #[max_len(MAX_NAME_LEN)]
    pub name: String,

    #[max_len(MAX_DESCRIPTION_LEN)]
    pub description: String,

    pub created_at: i64,

    #[max_len(MAX_REALM_MASTERS)]
    pub masters: Vec<RealmMaster>,
}
