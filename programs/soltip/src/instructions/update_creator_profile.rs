use anchor_lang::prelude::*;

use crate::states::{Profile, PROFILE_SEED};
use crate::errors::ProfileError;
use crate::events::UpdateProfileEvent;

pub fn update_creator_profile(
    ctx: Context<UpdateCreatorProfile>,
    name: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    about_me: Option<String>,
) -> Result<()> {
    require!(name.is_some() || email.is_some() || bio.is_some() || about_me.is_some(), ProfileError::NoArgumentProvided);

    ctx.accounts.profile.update(name, email, bio, about_me)?;

    emit!(UpdateProfileEvent {
        profile: ctx.accounts.profile.key(),
        creator: ctx.accounts.creator.key(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateCreatorProfile<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        mut,
        seeds = [PROFILE_SEED.as_bytes(), creator.key().as_ref()],
        bump
    )]
    profile: Account<'info, Profile>,
}
