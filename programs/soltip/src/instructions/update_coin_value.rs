use anchor_lang::prelude::*;

use crate::errors::ProfileError;
use crate::events::UpdateCoinValueEvent;
use crate::states::{Profile, PROFILE_SEED};

pub fn update_coin_value(ctx: Context<UpdateCoinValue>, value: u64) -> Result<()> {
    require!(value > 0, ProfileError::InvalidCoinValue);

    let profile = &mut ctx.accounts.profile;

    profile.coin_value = value;

    emit!(UpdateCoinValueEvent {
        profile: profile.key(),
        coin_value: value,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateCoinValue<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        mut,
        seeds = [PROFILE_SEED.as_bytes(), creator.key().as_ref()],
        bump
    )]
    profile: Account<'info, Profile>,
}
