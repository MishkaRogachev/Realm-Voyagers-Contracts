use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum LocationEventType {
    LocationAdded {
        name: String,
        tilemap: String,
        tileset: String,
    },
    LocationUpdated {
        name: String,
        tilemap: String,
        tileset: String,
    },
    LocationRemoved {},
}

#[event]
pub struct LocationEvent {
    pub event_type: LocationEventType,
    pub location_pubkey: Pubkey,
    pub realm_pubkey: Pubkey,
}
