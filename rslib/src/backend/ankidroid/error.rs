use crate::{
    error::{
        DbError, DbErrorKind as DB, FilteredDeckError, NetworkError, NetworkErrorKind as Net,
        SearchErrorKind, SyncError, SyncErrorKind as Sync,
    },
    prelude::AnkiError,
};

pub(super) fn debug_produce_error(s: &str) -> AnkiError {
    let error_value = "error_value".to_string();
    let err = match s {
        "InvalidInput" => AnkiError::InvalidInput(error_value),
        "TemplateError" => AnkiError::TemplateError(error_value),
        "IoError" => AnkiError::IoError(error_value),
        "DbErrorFileTooNew" => AnkiError::DbError(DbError {
            info: error_value,
            kind: DB::FileTooNew,
        }),
        "DbErrorFileTooOld" => AnkiError::DbError(DbError {
            info: error_value,
            kind: DB::FileTooOld,
        }),
        "DbErrorMissingEntity" => AnkiError::DbError(DbError {
            info: error_value,
            kind: DB::MissingEntity,
        }),
        "DbErrorCorrupt" => AnkiError::DbError(DbError {
            info: error_value,
            kind: DB::Corrupt,
        }),
        "DbErrorLocked" => AnkiError::DbError(DbError {
            info: error_value,
            kind: DB::Locked,
        }),
        "DbErrorOther" => AnkiError::DbError(DbError {
            info: error_value,
            kind: DB::Other,
        }),
        "NetworkError" => AnkiError::NetworkError(NetworkError {
            info: error_value,
            kind: Net::Offline,
        }),
        "SyncErrorConflict" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::Conflict,
        }),
        "SyncErrorServerError" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::ServerError,
        }),
        "SyncErrorClientTooOld" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::ClientTooOld,
        }),
        "SyncErrorAuthFailed" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::AuthFailed,
        }),
        "SyncErrorServerMessage" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::ServerMessage,
        }),
        "SyncErrorClockIncorrect" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::ClockIncorrect,
        }),
        "SyncErrorOther" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::Other,
        }),
        "SyncErrorResyncRequired" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::ResyncRequired,
        }),
        "SyncErrorDatabaseCheckRequired" => AnkiError::SyncError(SyncError {
            info: error_value,
            kind: Sync::DatabaseCheckRequired,
        }),
        "JSONError" => AnkiError::JsonError(error_value),
        "ProtoError" => AnkiError::ProtoError(error_value),
        "Interrupted" => AnkiError::Interrupted,
        "CollectionNotOpen" => AnkiError::CollectionNotOpen,
        "CollectionAlreadyOpen" => AnkiError::CollectionAlreadyOpen,
        "NotFound" => AnkiError::NotFound,
        "Existing" => AnkiError::Existing,
        "FilteredDeckError" => {
            AnkiError::FilteredDeckError(FilteredDeckError::FilteredDeckRequired)
        }
        "SearchError" => AnkiError::SearchError(SearchErrorKind::EmptyGroup),
        "FatalError" => AnkiError::FatalError(error_value),
        unknown => AnkiError::FatalError(format!("Unknown Error code: {}", unknown)),
    };
    err
}
