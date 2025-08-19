use anchor_lang::prelude::*;

#[event]
pub struct InitializeProfileEvent {
    pub profile: Pubkey,
    pub creator: Pubkey,
}

#[event]
pub struct UpdateProfileEvent {
    pub profile: Pubkey,
    pub creator: Pubkey,
}