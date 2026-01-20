/// Result of a value conversion process.
pub type Result<T> = std::result::Result<T, NotAlphabeticError>;

/// The generic error type for conversion.
#[derive(Debug, thiserror::Error, Default)]
#[error("invalid parameter (doesn't represent ASCII letter characters)")]
pub struct NotAlphabeticError;
