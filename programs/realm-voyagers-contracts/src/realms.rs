use anchor_lang::prelude::*;

const REALM_SEED: &[u8] = b"realm";
const LOCATION_SEED: &[u8] = b"location";

#[account]
#[derive(InitSpace)]
pub struct Realm {
    pub master: Pubkey,
    #[max_len(64)]
    pub name: String,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Location {
    pub realm: Pubkey, // Reference to parent realm
    #[max_len(64)]
    pub name: String,
    #[max_len(256)]
    pub tilemap_url: String,
    #[max_len(256)]
    pub tileset_url: String,
}

#[derive(Accounts)]
pub struct CreateRealm<'info> {
    #[account(mut)]
    pub master: Signer<'info>,

    #[account(
        init,
        payer = master,
        space = 8 + Realm::INIT_SPACE,
        seeds = [REALM_SEED, master.key().as_ref()],
        bump
    )]
    pub realm: Account<'info, Realm>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRealm<'info> {
    #[account(mut, has_one = master)]
    pub realm: Account<'info, Realm>,

    pub master: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddRealmLocation<'info> {
    #[account(mut, has_one = master)]
    pub realm: Account<'info, Realm>,

    #[account(
        init,
        payer = master,
        space = 8 + Location::INIT_SPACE,
        seeds = [LOCATION_SEED, realm.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub location: Account<'info, Location>,

    #[account(mut)]
    pub master: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRealmLocation<'info> {
    #[account(mut, has_one = master)]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub location: Account<'info, Location>,

    pub master: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveRealmLocation<'info> {
    #[account(mut, close = master)]
    pub location: Account<'info, Location>,

    #[account(mut, has_one = master)]
    pub realm: Account<'info, Realm>,

    pub master: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteRealm<'info> {
    #[account(mut, close = master, has_one = master)]
    pub realm: Account<'info, Realm>,

    #[account(mut)]
    pub master: Signer<'info>,
}
