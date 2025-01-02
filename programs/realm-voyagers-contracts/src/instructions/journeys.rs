use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::ErrorCode;
use crate::state::*;

#[derive(Accounts)]
#[instruction(realm_id: String)]
pub struct StartJourney<'info> {
    #[account(seeds = [REALM_SEED, realm_id.as_bytes()], bump)]
    pub realm: Account<'info, Realm>,

    #[account(
        init,
        payer = player,
        space = 8 + Journey::INIT_SPACE,
        seeds = [JOURNEY_SEED, realm_id.as_bytes(), player.key().as_ref()],
        bump
    )]
    pub journey: Account<'info, Journey>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn start_journey(ctx: Context<StartJourney>, _realm_id: String) -> Result<()> {
    let realm = &ctx.accounts.realm;

    require!(
        realm.starting_location.is_some(),
        ErrorCode::RealmHasNoStartingLocation
    );

    let journey = &mut ctx.accounts.journey;
    journey.realm = realm.key();
    journey.started_at = Clock::get()?.unix_timestamp;
    journey.player = *ctx.accounts.player.key;
    journey.location = realm.starting_location.unwrap();
    journey.position = realm.starting_position;

    Ok(())
}
