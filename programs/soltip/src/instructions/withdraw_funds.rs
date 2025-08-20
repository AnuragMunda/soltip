use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::{
    utils::errors::ProfileError,
    states::{Profile, PROFILE_SEED},
};

pub fn withdraw_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    let creator = &ctx.accounts.creator;
    let program = ctx.accounts.system_program.to_account_info();
    let creator_key = creator.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        PROFILE_SEED.as_bytes(),
        creator_key.as_ref(),
        &[ctx.bumps.profile],
    ]];
    let lamports = profile.get_lamports();

    require!(creator_key == profile.creator, ProfileError::Unauthorized);
    require!(lamports > 0, ProfileError::InsufficientFunds);

    let cpi_context = CpiContext::new(
        program,
        Transfer {
            from: profile.to_account_info(),
            to: creator.to_account_info(),
        },
    )
    .with_signer(signer_seeds);

    transfer(cpi_context, lamports)?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        mut,
        seeds = [PROFILE_SEED.as_bytes(), creator.key().as_ref()],
        bump
    )]
    profile: Account<'info, Profile>,

    pub system_program: Program<'info, System>,
}
