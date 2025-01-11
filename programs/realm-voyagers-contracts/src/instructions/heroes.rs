use anchor_lang::prelude::*;

use crate::constants::*;
use crate::events::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(hero_id: String, description: HeroDescription)]
pub struct CreateHero<'info> {
    #[account(
        init,
        payer = player,
        space = crate::hero_space!(description, 0, 0),
        seeds = [HERO_SEED, player.key().as_ref(), hero_id.as_bytes()],
        bump
    )]
    pub hero: Account<'info, Hero>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_hero(
    ctx: Context<CreateHero>,
    _hero_id: String,
    description: HeroDescription,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;

    let hero = &mut ctx.accounts.hero;
    hero.owner = *ctx.accounts.player.key;
    hero.description = description.clone();
    // TODO: Add tags
    // hero.stats = tags
    //     .iter()
    //     .fold(HeroStats::zero(), |stats, tag| stats + tag.base_stats());
    // hero.tags = tags;
    hero.level = 1;
    hero.xp = 0;
    hero.items = vec![];
    hero.created_at = now;
    hero.updated_at = now;

    emit!(HeroEvent {
        player: *ctx.accounts.player.key,
        hero_pubkey: hero.key(),
        event_type: HeroEventType::HeroCreated { description },
    });

    Ok(())
}
