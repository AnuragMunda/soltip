use anchor_lang::prelude::*;

#[error_code]
pub enum InitializeProfileError {
    #[msg("Name exceeds maximum length")]
    NameTooLong,
    #[msg("Email exceeds maximum length")]
    EmailTooLong,
    #[msg("Bio exceeds maximum length")]
    BioTooLong,
    #[msg("About me exceeds maximum length")]
    AboutMeTooLong,
}
