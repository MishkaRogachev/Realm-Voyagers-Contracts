use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized realm master")]
    UnauthorizedRealmMaster,

    #[msg("Cant remove realm owner")]
    CantRemoveRealmOwner,

    #[msg("Realm master not found")]
    RealmMasterNotFound,

    #[msg("Duplicate realm master")]
    DuplicateRealmMaster,

    #[msg("Name is too long")]
    NameTooLong,

    #[msg("Deatils is too long")]
    DetailsTooLong,

    #[msg("Resource path is too long")]
    ResourcePathTooLong,

    #[msg("Dimension was not provided")]
    DimensionNotProvided,

    #[msg("Realm has no starting dimension")]
    RealmHasNoStartingDimension,
}
