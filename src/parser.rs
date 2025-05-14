use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{escaped, take_till, take_while1},
    character::complete::{char, none_of, one_of},
    combinator::map_res,
    error::{ErrorKind, Error as NomError},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    Err as NomErr, IResult,
};

use crate::error::{Error, Result};
use crate::sections::{
    AccessMode, Section, SourceType, StructureData, UCDF,
};
use crate::types::{Endpoint, Field};

/// Function to parse a UCDF string into a UCDF structure
pub fn parse(s: &str) -> Result<UCDF> {
    match ucdf_parser(s) {
        Ok((_, ucdf)) => Ok(ucdf),
        Err(err) => {
            match err {
                NomErr::Incomplete(_) => Err(Error::InvalidFormat("Incomplete input".to_string())),
                NomErr::Error(e) => Err(Error::InvalidFormat(format!("Parser error: {:?}", e.code))),
                NomErr::Failure(e) => {
                    if e.code == ErrorKind::Tag {
                        // For specific errors like invalid access mode
                        Err(Error::InvalidAccessMode(format!("Invalid input at: {}", s)))
                    } else {
                        Err(Error::InvalidFormat(format!("Parser failure: {:?}", e.code)))
                    }
                },
            }
        }
    }
}

// Primary parser for UCDF strings
fn ucdf_parser(input: &str) -> IResult<&str, UCDF> {
    let (input, sections) = separated_list0(char(';'), section_parser)(input)?;

    // Extract and validate type section
    let type_section = sections.iter().find_map(|section| {
        if let Section::Type(source_type) = section {
            Some(source_type.clone())
        } else {
            None
        }
    });

    let source_type = match type_section {
        Some(source_type) => source_type,
        None => return Err(NomErr::Error(NomError::new(input, ErrorKind::Tag))),
    };

    // Create base UCDF with type
    let mut ucdf = UCDF::builder().source_type(source_type).build();

    // Process all sections
    for section in sections {
        match section {
            Section::Type(_) => {} // Already handled
            Section::Connection(key, value) => {
                ucdf.add_connection(&key, &value);
            }
            Section::Structure(key, structure) => match structure {
                StructureData::Fields(fields) => {
                    ucdf.add_fields(fields);
                }
                StructureData::Endpoints(endpoints) => {
                    ucdf.add_endpoints(endpoints);
                }
                StructureData::Format(format) => {
                    ucdf.add_format(&format);
                }
                StructureData::Custom(_, value) => {
                    ucdf.add_custom_structure(&key, &value);
                }
            },
            Section::Access(access_mode) => {
                ucdf.set_access_mode(access_mode);
            }
            Section::Meta(key, value) => {
                ucdf.add_metadata(&key, &value);
            }
        }
    }

    Ok((input, ucdf))
}

// Parse a section: key=value
fn section_parser(input: &str) -> IResult<&str, Section> {
// Parse key=value pair, returning error if format is invalid
let (input, (key, value)) = separated_pair(
    key_parser,
    char('='),
    alt((quoted_value_parser, simple_value_parser)),
)(input)?;
    
// Check if the key is non-empty
if key.is_empty() {
    return Err(NomErr::Error(NomError::new(input, ErrorKind::Tag)));
}

let result = if key == "t" {
        // Type section
        match SourceType::from_str(value) {
            Ok(source_type) => Section::Type(source_type),
            Err(_) => return Err(NomErr::Error(NomError::new(input, ErrorKind::Tag))),
        }
    } else if let Some(conn_key) = key.strip_prefix("c.") {
        // Connection section
        Section::Connection(conn_key.to_string(), value.to_string())
    } else if let Some(struct_key) = key.strip_prefix("s.") {
        // Structure section
        match struct_key {
            "fields" => {
                let (_, fields) = parse_fields(value)?;
                Section::Structure(struct_key.to_string(), StructureData::Fields(fields))
            }
            "endpoints" => {
                let (_, endpoints) = parse_endpoints(value)?;
                Section::Structure(struct_key.to_string(), StructureData::Endpoints(endpoints))
            }
            "format" => Section::Structure(
                struct_key.to_string(),
                StructureData::Format(value.to_string()),
            ),
            _ => Section::Structure(
                struct_key.to_string(),
                StructureData::Custom(struct_key.to_string(), value.to_string()),
            ),
        }
    } else if key == "a" {
        // Access mode section
        match AccessMode::from_str(value) {
            Ok(access_mode) => Section::Access(access_mode),
            Err(_) => return Err(NomErr::Failure(NomError::new(input, ErrorKind::Tag))),
        }
    } else if let Some(meta_key) = key.strip_prefix("m.") {
        // Metadata section
        Section::Meta(meta_key.to_string(), value.to_string())
    } else {
        return Err(NomErr::Error(NomError::new(input, ErrorKind::Tag)));
    };

    Ok((input, result))
}

// Key parser: any character except '=' and ';'
fn key_parser(input: &str) -> IResult<&str, &str> {
    take_while1(|c| c != '=' && c != ';')(input)
}

// Simple value parser: any character except ';'
fn simple_value_parser(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == ';')(input)
}

// Parse a quoted string value
fn quoted_value_parser(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        escaped(none_of("\\\""), '\\', one_of("\"\\nrt")),
        char('"'),
    )(input)
}

// Helper function to parse fields
fn parse_fields(input: &str) -> IResult<&str, Vec<Field>> {
    separated_list0(
        char::<&str, nom::error::Error<&str>>(','),
        map_res(
            separated_pair(
                take_while1(|c| c != ':' && c != ',' && c != ';'),
                char::<&str, nom::error::Error<&str>>(':'),
                take_while1(|c| c != ',' && c != ';'),
            ),
            |(name, dtype)| -> Result<Field> {
                Ok(Field::builder()
                    .name(name.to_string())
                    .dtype(dtype.to_string())
                    .build())
            },
        ),
    )(input)
}

// Helper function to parse endpoints
fn parse_endpoints(input: &str) -> IResult<&str, Vec<Endpoint>> {
    separated_list0(
        char::<&str, nom::error::Error<&str>>(','),
        map_res(
            separated_pair(
                take_while1(|c| c != ':' && c != ',' && c != ';'),
                char::<&str, nom::error::Error<&str>>(':'),
                take_while1(|c| c != ',' && c != ';'),
            ),
            |(path, method)| -> Result<Endpoint> {
                Ok(Endpoint::builder()
                    .path(path.to_string())
                    .method(method.to_string())
                    .build())
            },
        ),
    )(input)
}

/// Parser for UCDF strings
pub struct Parser;

impl Parser {
    /// Create a new Parser
    pub fn new() -> Self {
        Parser
    }

    /// Parse a UCDF string into a UCDF structure
    pub fn parse(&self, s: &str) -> Result<UCDF> {
        parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sections::*;

    #[test]
    fn test_parse_csv_file() {
        let ucdf_str = "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str,email:str;a=r;m.desc=User data";
        let ucdf = parse(ucdf_str).unwrap();

        assert_eq!(ucdf.source_type.category, "file");
        assert_eq!(ucdf.source_type.subtype, Some("csv".to_string()));
        assert_eq!(
            ucdf.connection.get("path"),
            Some(&"/data/users.csv".to_string())
        );
        assert_eq!(ucdf.access_mode, Some(AccessMode::Read));
        assert_eq!(ucdf.metadata.get("desc"), Some(&"User data".to_string()));

        if let Some(StructureData::Fields(fields)) = ucdf.structure.get("fields") {
            assert_eq!(fields.len(), 3);
            assert_eq!(fields[0].name, "id");
            assert_eq!(fields[0].dtype, "int");
            assert_eq!(fields[1].name, "name");
            assert_eq!(fields[1].dtype, "str");
        } else {
            panic!("Expected fields structure");
        }
    }

    #[test]
    fn test_parse_postgresql() {
        let ucdf_str = "t=db.postgresql;c.host=db.prod;c.user=readonly;c.db=sales;s.fields=id:int,amount:float,date:date;a=r";
        let ucdf = parse(ucdf_str).unwrap();

        assert_eq!(ucdf.source_type.category, "db");
        assert_eq!(ucdf.source_type.subtype, Some("postgresql".to_string()));
        assert_eq!(ucdf.connection.get("host"), Some(&"db.prod".to_string()));
        assert_eq!(ucdf.connection.get("user"), Some(&"readonly".to_string()));
        assert_eq!(ucdf.connection.get("db"), Some(&"sales".to_string()));
        assert_eq!(ucdf.access_mode, Some(AccessMode::Read));

        if let Some(StructureData::Fields(fields)) = ucdf.structure.get("fields") {
            assert_eq!(fields.len(), 3);
            assert_eq!(fields[0].name, "id");
            assert_eq!(fields[0].dtype, "int");
            assert_eq!(fields[1].name, "amount");
            assert_eq!(fields[1].dtype, "float");
        } else {
            panic!("Expected fields structure");
        }
    }

    #[test]
    fn test_quoted_values() {
        let ucdf_str = "t=file.csv;c.path=\"/data/My Documents/file.csv\";m.desc=\"User, data; with special=chars\"";
        let ucdf = parse(ucdf_str).unwrap();

        assert_eq!(
            ucdf.connection.get("path"),
            Some(&"/data/My Documents/file.csv".to_string())
        );
        assert_eq!(
            ucdf.metadata.get("desc"),
            Some(&"User, data; with special=chars".to_string())
        );
    }

    #[test]
    fn test_empty_sections() {
        // Empty sections should be ignored
        let ucdf_str = "t=file.csv;;";
        let ucdf = parse(ucdf_str).unwrap();
        assert_eq!(ucdf.source_type.category, "file");
        // Confirm that empty sections are parsed correctly
        assert!(ucdf.connection.0.is_empty());
    }

    #[test]
    fn test_missing_type() {
        // Missing type section should return error
        let ucdf_str = "c.path=/data.csv";
        assert!(parse(ucdf_str).is_err());
    }

    #[test]
    fn test_multiple_sections() {
        let ucdf_str = "t=db.postgresql;c.host=localhost;c.port=5432;c.user=postgres;s.fields=id:int,name:str;a=rw;m.desc=Test database";
        let ucdf = parse(ucdf_str).unwrap();
        
        // Check source type
        assert_eq!(ucdf.source_type.category, "db");
        assert_eq!(ucdf.source_type.subtype, Some("postgresql".to_string()));
        
        // Check connection parameters
        assert_eq!(ucdf.connection.get("host"), Some(&"localhost".to_string()));
        assert_eq!(ucdf.connection.get("port"), Some(&"5432".to_string()));
        assert_eq!(ucdf.connection.get("user"), Some(&"postgres".to_string()));
        
        // Check fields
        if let Some(StructureData::Fields(fields)) = ucdf.structure.get("fields") {
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "id");
            assert_eq!(fields[0].dtype, "int");
            assert_eq!(fields[1].name, "name");
            assert_eq!(fields[1].dtype, "str");
        } else {
            panic!("Expected fields structure");
        }
        
        // Check access mode
        assert_eq!(ucdf.access_mode, Some(AccessMode::ReadWrite));
        
        // Check metadata
        assert_eq!(ucdf.metadata.get("desc"), Some(&"Test database".to_string()));
    }

    #[test]
    fn test_special_characters() {
        let ucdf_str = "t=file.csv;c.path=\"/path/with spaces/and;special=chars.csv\";m.desc=\"Line 1\\nLine 2\"";
        let ucdf = parse(ucdf_str).unwrap();
        
        assert_eq!(ucdf.source_type.category, "file");
        assert_eq!(ucdf.source_type.subtype, Some("csv".to_string()));
        
        // Check that special characters in quoted values are preserved
        assert_eq!(
            ucdf.connection.get("path"),
            Some(&"/path/with spaces/and;special=chars.csv".to_string())
        );
        
        // Check that escaped characters are handled correctly
        assert_eq!(
            ucdf.metadata.get("desc"),
            Some(&"Line 1\\nLine 2".to_string())
        );
    }

    #[test]
    fn test_complex_structure() {
        let ucdf_str = "t=stream.kafka;c.brokers=server1:9092,server2:9092;s.format=json;s.fields=id:str,timestamp:datetime,data:json;a=r";
        let ucdf = parse(ucdf_str).unwrap();
        
        assert_eq!(ucdf.source_type.category, "stream");
        assert_eq!(ucdf.source_type.subtype, Some("kafka".to_string()));
        
        // Check that both structure sections are parsed
        if let Some(StructureData::Format(format)) = ucdf.structure.get("format") {
            assert_eq!(format, "json");
        } else {
            panic!("Expected format structure");
        }
        
        if let Some(StructureData::Fields(fields)) = ucdf.structure.get("fields") {
            assert_eq!(fields.len(), 3);
            assert_eq!(fields[0].name, "id");
            assert_eq!(fields[0].dtype, "str");
            assert_eq!(fields[1].name, "timestamp");
            assert_eq!(fields[1].dtype, "datetime");
            assert_eq!(fields[2].name, "data");
            assert_eq!(fields[2].dtype, "json");
        } else {
            panic!("Expected fields structure");
        }
    }

    #[test]
    fn test_malformed_input() {
        // Test invalid access mode (should be caught by AccessMode::from_str)
        assert!(parse("t=file.csv;a=invalid").is_err());
        
        // Test missing type section
        assert!(parse("c.path=/data.csv").is_err());
        
        // Test completely invalid format
        assert!(parse("not a valid ucdf string").is_err());
    }
}