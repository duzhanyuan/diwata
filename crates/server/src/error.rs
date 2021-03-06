use diwata_intel::error::IntelError;
use rustorm::error::DbError;

#[derive(Debug)]
pub enum ServiceError {
    GenericError(String),
    PoolResourceError,
    CacheLockError,
    IntelError(IntelError),
    DbError(DbError),
    NoDbUrlSpecified,
    NotFound,
    RequiredCredentialsNotFound,
}

impl From<DbError> for ServiceError {
    fn from(e: DbError) -> ServiceError {
        ServiceError::DbError(e)
    }
}

impl From<IntelError> for ServiceError {
    fn from(e: IntelError) -> ServiceError {
        ServiceError::IntelError(e)
    }
}
