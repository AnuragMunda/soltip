use anchor_lang::prelude::*;

use crate::errors::ProfileError;
use crate::events::InitializeProfileEvent;
use crate::states::profile::*;

pub fn initialize_creator_profile(
    ctx: Context<InitializeCreatorProfile>,
    name: String,
    email: String,
    bio: String,
    about_me: String,
) -> Result<()> {
    let creator_profile = &mut ctx.accounts.profile;
    let creator_key = ctx.accounts.creator.key();

    require!(
        name.len() <= NAME_LENGTH,
        ProfileError::NameTooLong
    );
    require!(
        email.len() <= EMAIL_LENGTH,
        ProfileError::NameTooLong
    );
    require!(bio.len() <= BIO_LENGTH, ProfileError::NameTooLong);
    require!(
        about_me.len() <= ABOUT_ME_LENGTH,
        ProfileError::NameTooLong
    );

    creator_profile.creator = creator_key;
    creator_profile.name = name.clone();
    creator_profile.email = email;
    creator_profile.bio = bio;
    creator_profile.about_me = about_me;

    emit!(InitializeProfileEvent {
        profile: creator_profile.key(),
        creator: creator_key,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCreatorProfile<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Profile::INIT_SPACE,
        seeds = [PROFILE_SEED.as_bytes(), creator.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    pub system_program: Program<'info, System>,
}
