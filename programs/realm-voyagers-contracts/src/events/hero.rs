use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum HeroEventType {
    HeroCreated { hero_pubkey: Pubkey },
    HeroUpdated { hero_pubkey: Pubkey },
}

#[event]
pub struct HeroEvent {
    pub event_type: HeroEventType,
}
