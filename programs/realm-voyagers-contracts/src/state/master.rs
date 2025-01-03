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

impl RealmMaster {
    pub fn can_update_realm(&self) -> bool {
        matches!(self.role, RealmMasterRole::Owner | RealmMasterRole::Admin)
    }

    pub fn can_delete_realm(&self) -> bool {
        matches!(self.role, RealmMasterRole::Owner)
    }

    pub fn can_manage_realm_masters(&self) -> bool {
        matches!(self.role, RealmMasterRole::Owner)
    }

    pub fn can_add_realm_dimension(&self) -> bool {
        matches!(self.role, RealmMasterRole::Owner | RealmMasterRole::Admin)
    }

    pub fn can_manage_realm_dimension(&self, dimension: &crate::state::RealmDimension) -> bool {
        match self.role {
            RealmMasterRole::Owner => true,
            RealmMasterRole::Admin => dimension.owner == self.pubkey,
            RealmMasterRole::Curator => false,
        }
    }

    pub fn can_set_realm_starting_point(&self) -> bool {
        matches!(self.role, RealmMasterRole::Owner)
    }
}
