use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum RealmDimensionEventType {
    DimensionAdded { name: String },
    DimensionUpdated { name: String },
    DimensionRemoved {},
}

#[event]
pub struct RealmDimensionEvent {
    pub event_type: RealmDimensionEventType,
    pub dimension_pubkey: Pubkey,
    pub realm_pubkey: Pubkey,
}
