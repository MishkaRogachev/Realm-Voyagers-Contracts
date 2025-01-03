use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Journey {
    pub realm: Pubkey,
    pub player: Pubkey,
    pub started_at: i64,

    pub dimension: Pubkey,
    pub position: crate::state::Position,
}
