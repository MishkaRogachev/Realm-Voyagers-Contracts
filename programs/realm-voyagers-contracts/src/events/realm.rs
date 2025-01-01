use anchor_lang::prelude::*;

use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum RealmEventType {
    RealmCreated {
        name: String,
        description: String,
    },
    RealmUpdated {
        name: String,
        description: String,
    },
    RealmDeleted {},

    RealmMasterAdded {
        master: RealmMaster,
    },
    RealmMasterRemoved {
        master: RealmMaster,
    },
    RealmOwnershipTransferred {
        old_owner: RealmMaster,
        new_owner: RealmMaster,
    },
}

#[event]
pub struct RealmEvent {
    pub event_type: RealmEventType,
    pub realm_pubkey: Pubkey,
}
