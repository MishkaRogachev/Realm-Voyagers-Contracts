use anchor_lang::prelude::*;

use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum HeroEventType {
    HeroCreated { description: HeroDescription },
}

#[event]
pub struct HeroEvent {
    pub event_type: HeroEventType,
    pub hero_pubkey: Pubkey,
    pub player: Pubkey,
}
