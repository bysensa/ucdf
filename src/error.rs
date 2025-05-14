use thiserror::Error;

/// Result type for UCDF operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error enum for UCDF parsing and operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing required type section (t=...)")]
    MissingTypeSection,

    #[error("Invalid section format: {0}")]
    InvalidSectionFormat(String),

    #[error("Invalid source type format: {0}")]
    InvalidSourceType(String),

    #[error("Invalid access mode: {0}")]
    InvalidAccessMode(String),

    #[error("Invalid field format: {0}")]
    InvalidFieldFormat(String),

    #[error("Invalid endpoint format: {0}")]
    InvalidEndpointFormat(String),

    #[error("Invalid type declaration: {0}")]
    InvalidTypeDeclaration(String),

    #[error("Unknown section prefix: {0}")]
    UnknownSectionPrefix(String),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Nom parsing error: {0}")]
    NomError(String),
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => Error::NomError("Incomplete input".to_string()),
            nom::Err::Error(e) => Error::NomError(format!("Parser error at: {}", e.input)),
            nom::Err::Failure(e) => Error::NomError(format!("Fatal error at: {}", e.input)),
        }
    }
}