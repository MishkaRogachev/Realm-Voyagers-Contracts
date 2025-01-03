use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum DimensionEventType {
    DimensionAdded {
        name: String,
        tilemap: String,
        tileset: String,
    },
    DimensionUpdated {
        name: String,
        tilemap: String,
        tileset: String,
    },
    DimensionRemoved {},
}

#[event]
pub struct DimensionEvent {
    pub event_type: DimensionEventType,
    pub dimension_pubkey: Pubkey,
    pub realm_pubkey: Pubkey,
}
