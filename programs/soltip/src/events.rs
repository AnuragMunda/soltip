use anchor_lang::prelude::*;

#[event]
pub struct InitializeProfileEvent {
    pub profile: Pubkey,
    pub creator: Pubkey,
    pub name: String,
}