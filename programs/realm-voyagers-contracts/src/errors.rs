use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized realm master")]
    UnauthorizedRealmMaster,

    #[msg("Name is too long")]
    NameTooLong,

    #[msg("Description is too long")]
    DescriptionTooLong,

    #[msg("Resource path is too long")]
    ResourcePathTooLong,
}
