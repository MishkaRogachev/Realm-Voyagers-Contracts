use anchor_lang::prelude::*;

use crate::constants::*;
use crate::events::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(hero_id: String, description: HeroDescription)]
pub struct CreateHero<'info> {
    #[account(
        init,
        payer = master,
        space = crate::hero_space!(description),
        seeds = [HERO_SEED, master.key().as_ref(), hero_id.as_bytes()],
        bump
    )]
    pub hero: Account<'info, Hero>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_hero(
    ctx: Context<CreateHero>,
    _hero_id: String,
    description: HeroDescription,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;

    let hero = &mut ctx.accounts.hero;
    hero.master = *ctx.accounts.master.key;
    hero.description = description;
    // TODO: hero stats
    hero.created_at = now;
    hero.updated_at = now;

    emit!(HeroEvent {
        event_type: HeroEventType::HeroCreated {
            hero_pubkey: hero.key()
        },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(hero_id: String, description: HeroDescription)]
pub struct UpdateHeroDescription<'info> {
    #[account(
        mut,
        seeds = [HERO_SEED, master.key().as_ref(), hero_id.as_bytes()],
        bump,
        realloc = crate::hero_space!(description),
        realloc::payer = master,
        realloc::zero = false
    )]
    pub hero: Account<'info, Hero>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_hero_description(
    ctx: Context<UpdateHeroDescription>,
    _hero_id: String,
    description: HeroDescription,
) -> Result<()> {
    let hero = &mut ctx.accounts.hero;
    hero.description = description;
    hero.updated_at = Clock::get()?.unix_timestamp;

    emit!(HeroEvent {
        event_type: HeroEventType::HeroUpdated {
            hero_pubkey: hero.key()
        },
    });

    Ok(())
}

// #[derive(Accounts)]
// #[instruction(hero_id: String, description: HeroDescription)]
// pub struct CreateHero<'info> {
//     #[account(
//         init,
//         payer = player,
//         space = crate::player_hero_space!(description, 0, 0),
//         seeds = [HERO_SEED, player.key().as_ref(), hero_id.as_bytes()],
//         bump
//     )]
//     pub hero: Account<'info, PlayerHero>,

//     #[account(mut)]
//     pub player: Signer<'info>,

//     pub system_program: Program<'info, System>,
// }

// pub fn create_hero(
//     ctx: Context<CreateHero>,
//     _hero_id: String,
//     description: HeroDescription,
// ) -> Result<()> {
//     let now = Clock::get()?.unix_timestamp;

//     let hero = &mut ctx.accounts.hero;
//     hero.player = *ctx.accounts.player.key;
//     hero.description = description.clone();
//     // TODO: Add tags
//     // hero.stats = tags
//     //     .iter()
//     //     .fold(HeroStats::zero(), |stats, tag| stats + tag.base_stats());
//     // hero.tags = tags;
//     hero.level = 1;
//     hero.xp = 0;
//     hero.items = vec![];
//     hero.created_at = now;
//     hero.updated_at = now;

//     emit!(HeroEvent {
//         player: *ctx.accounts.player.key,
//         hero_pubkey: hero.key(),
//         event_type: HeroEventType::HeroCreated { description },
//     });

//     Ok(())
// }
