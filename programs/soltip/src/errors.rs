use anchor_lang::prelude::*;

#[error_code]
pub enum ProfileError {
    #[msg("Name exceeds maximum length")]
    NameTooLong,
    #[msg("Email exceeds maximum length")]
    EmailTooLong,
    #[msg("Bio exceeds maximum length")]
    BioTooLong,
    #[msg("About me exceeds maximum length")]
    AboutMeTooLong,
    #[msg("No argument is provided")]
    NoArgumentProvided,
    #[msg("Coin value must be greater than 0")]
    InvalidCoinValue,
}
