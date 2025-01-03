use anchor_lang::prelude::*;

use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum RealmEventType {
    RealmCreated {
        description: RealmDescription,
    },
    RealmDescriptionUpdated {
        description: RealmDescription,
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
