//! UCDF - Unified Compact Data Format
//! 
//! A Rust implementation of the UCDF format for describing data sources in a single line.
//! UCDF consists of sections separated by semicolons (`;`), where each section contains
//! key-value pairs representing different aspects of a data source.
//!
//! # Examples
//!
//! ```
//! use ucdf::{parse, UCDF, SourceType, AccessMode, Field};
//!
//! // Parse a UCDF string
//! let ucdf_str = "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str;a=r";
//! let ucdf = ucdf::parse(ucdf_str).unwrap();
//! println!("Source type: {}", ucdf.source_type);
//!
//! // Create a UCDF object using builder pattern
//! let source_type = SourceType::builder()
//!     .category("db".to_string())
//!     .subtype(Some("postgresql".to_string()))
//!     .build();
//!
//! let ucdf = UCDF::builder()
//!     .source_type(source_type)
//!     .build()
//!     .with_connection("host", "localhost")
//!     .with_connection("port", "5432")
//!     .with_access_mode(AccessMode::ReadWrite);
//!
//! // Convert back to string
//! let ucdf_str = ucdf.to_string();
//! ```

mod error;
mod parser;
mod sections;
mod types;

pub use error::{Error, Result};
pub use parser::{parse, Parser};
pub use sections::{
    AccessMode, ConnectionParams, DataType, Metadata, Section, SourceType, StructureData, UCDF,
};
pub use types::{DataValue, Endpoint, Field};

// Re-export nom for public use
pub use nom;

/// Parse a UCDF string into a UCDF structure
///
/// # Examples
///
/// ```
/// use ucdf::parse;
///
/// let ucdf_str = "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str;a=r";
/// let result = parse(ucdf_str);
/// assert!(result.is_ok());
/// ```
pub fn from_str(s: &str) -> Result<UCDF> {
    parse(s)
}

/// Convert a UCDF structure to a string
///
/// # Examples
///
/// ```
/// use ucdf::{parse, UCDF};
///
/// let ucdf_str = "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str;a=r";
/// let ucdf: UCDF = parse(ucdf_str).unwrap();
/// let result = ucdf.to_string();
/// ```
pub fn to_string(ucdf: &UCDF) -> String {
    ucdf.to_string()
}

/// Re-export the `bon` crate for convenient access to the builder macros
pub use bon;

/// Parse UCDF with the Nom-based parser directly
/// Parse a UCDF string into a UCDF structure using the Nom-based parser directly.
/// 
/// This is an advanced function that uses the Nom-based parser directly.
/// Most users should use the `parse` function instead.
/// 
/// # Examples
/// 
/// ```
/// use ucdf::parse;
/// 
/// let ucdf_str = "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str;a=r";
/// let result = parse(ucdf_str);
/// assert!(result.is_ok());
/// ```
pub fn parse_with_nom(s: &str) -> Result<UCDF> {
    from_str(s)
}
