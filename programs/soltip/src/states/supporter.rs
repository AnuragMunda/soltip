use anchor_lang::prelude::*;

use crate::utils::errors::SupporterError;

pub const SUPPORTER_NAME_LENGTH: usize = 25;
pub const MESSAGE_LENGTH: usize = 25;

pub const SUPPORTER_SEED: &str = "SUPPORTER";

#[account]
#[derive(InitSpace)]
pub struct Supporter {
    supporter: Pubkey,
    #[max_len(SUPPORTER_NAME_LENGTH)]
    name: Option<String>,
    tip: u64,
    #[max_len(MESSAGE_LENGTH)]
    message: Option<String>,
}

impl Supporter {
    pub fn validateAndSet(&mut self, name: Option<String>, message: Option<String>) -> Result<()> {
        if let Some(name) = name {
            require!(
                name.len() <= SUPPORTER_NAME_LENGTH,
                SupporterError::NameTooLong
            );
            self.name = Some(name);
        }
        if let Some(message) = message {
            require!(
                message.len() <= MESSAGE_LENGTH,
                SupporterError::MessageTooLong
            );
            self.message = Some(message);
        }

        Ok(())
    }
}
