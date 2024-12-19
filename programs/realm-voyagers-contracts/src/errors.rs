use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("No locations provided")]
    NoLocationsProvieded,
    #[msg("Starting location is out range of provided locations")]
    StartingLocationOutOfRange,
}
