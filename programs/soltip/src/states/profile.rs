use anchor_lang::prelude::*;

pub const NAME_LENGTH: usize = 25;
pub const BIO_LENGTH: usize = 100;
pub const EMAIL_LENGTH: usize = 64;
pub const ABOUT_ME_LENGTH: usize = 500;
// pub const MAX_SOCIAL_LINKS: usize = 7;
// pub const SOCIAL_LINK_LENGTH: usize = 64;

pub const PROFILE_SEED: &str = "PROFILE";

#[account]
#[derive(InitSpace)]
pub struct Game {
    pub signer: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub creator: Pubkey,
    #[max_len(NAME_LENGTH)]
    pub name: String,
    #[max_len(EMAIL_LENGTH)]
    pub email: String,
    #[max_len(BIO_LENGTH)]
    pub bio: String,
    #[max_len(ABOUT_ME_LENGTH)]
    pub about_me: String,
    // #[max_len(MAX_SOCIAL_LINKS, SOCIAL_LINK_LENGTH)]
    // pub social_links: Vec<String>,
}
