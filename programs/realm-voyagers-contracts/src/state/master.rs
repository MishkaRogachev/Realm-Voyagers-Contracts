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
