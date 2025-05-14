use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use bon::bon;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::types::{Endpoint, Field};

/// Represents a source type in UCDF
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceType {
    pub category: String,
    pub subtype: Option<String>,
}

#[bon]
impl SourceType {
    #[builder]
    pub fn builder(category: String, subtype: Option<String>) -> Self {
        Self { category, subtype }
    }

    pub fn new(category: String, subtype: Option<String>) -> Self {
        Self { category, subtype }
    }
}

impl FromStr for SourceType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        match parts.len() {
            1 => Ok(SourceType {
                category: parts[0].to_string(),
                subtype: None,
            }),
            2 => Ok(SourceType {
                category: parts[0].to_string(),
                subtype: Some(parts[1].to_string()),
            }),
            _ => Err(Error::InvalidSourceType(s.to_string())),
        }
    }
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subtype {
            Some(subtype) => write!(f, "{}.{}", self.category, subtype),
            None => write!(f, "{}", self.category),
        }
    }
}

/// Access mode for UCDF sources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessMode {
    Read,
    Write,
    ReadWrite,
}

impl FromStr for AccessMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "r" => Ok(AccessMode::Read),
            "w" => Ok(AccessMode::Write),
            "rw" | "wr" => Ok(AccessMode::ReadWrite),
            _ => Err(Error::InvalidAccessMode(s.to_string())),
        }
    }
}

impl fmt::Display for AccessMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessMode::Read => write!(f, "r"),
            AccessMode::Write => write!(f, "w"),
            AccessMode::ReadWrite => write!(f, "rw"),
        }
    }
}

/// Represents the data type for fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime,
    Json,
    Custom(String),
}

impl FromStr for DataType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "str" => Ok(DataType::String),
            "int" => Ok(DataType::Integer),
            "float" => Ok(DataType::Float),
            "bool" => Ok(DataType::Boolean),
            "date" => Ok(DataType::Date),
            "datetime" => Ok(DataType::DateTime),
            "json" => Ok(DataType::Json),
            _ => Ok(DataType::Custom(s.to_string())),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::String => write!(f, "str"),
            DataType::Integer => write!(f, "int"),
            DataType::Float => write!(f, "float"),
            DataType::Boolean => write!(f, "bool"),
            DataType::Date => write!(f, "date"),
            DataType::DateTime => write!(f, "datetime"),
            DataType::Json => write!(f, "json"),
            DataType::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Structure data section which can contain different schema types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructureData {
    Fields(Vec<Field>),
    Endpoints(Vec<Endpoint>),
    Format(String),
    Custom(String, String),
}

/// Connection parameters section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionParams(pub HashMap<String, String>);

impl ConnectionParams {
    pub fn new() -> Self {
        ConnectionParams(HashMap::new())
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        self.0.insert(key.to_string(), value.to_string())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.0.iter()
    }
}

impl From<HashMap<String, String>> for ConnectionParams {
    fn from(map: HashMap<String, String>) -> Self {
        ConnectionParams(map)
    }
}

/// Metadata section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata(pub HashMap<String, String>);

impl Metadata {
    pub fn new() -> Self {
        Metadata(HashMap::new())
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        self.0.insert(key.to_string(), value.to_string())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.0.iter()
    }
}

impl From<HashMap<String, String>> for Metadata {
    fn from(map: HashMap<String, String>) -> Self {
        Metadata(map)
    }
}

/// UCDF Section enum representing different parts of a UCDF string
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Section {
    Type(SourceType),
    Connection(String, String),
    Structure(String, StructureData),
    Access(AccessMode),
    Meta(String, String),
}

/// Main UCDF structure that represents a UCDF data source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UCDF {
    pub source_type: SourceType,
    pub connection: ConnectionParams,
    pub structure: HashMap<String, StructureData>,
    pub access_mode: Option<AccessMode>,
    pub metadata: Metadata,
}

#[bon]
impl UCDF {
    #[builder]
    pub fn builder(
        source_type: SourceType,
        #[builder(default = ConnectionParams::new())] connection: ConnectionParams,
        #[builder(default = HashMap::new())] structure: HashMap<String, StructureData>,
        access_mode: Option<AccessMode>,
        #[builder(default = Metadata::new())] metadata: Metadata,
    ) -> Self {
        Self {
            source_type,
            connection,
            structure,
            access_mode,
            metadata,
        }
    }
    pub fn with_source_type(source_type: SourceType) -> Self {
        Self {
            source_type,
            connection: ConnectionParams::new(),
            structure: Default::default(),
            access_mode: None,
            metadata: Metadata::new(),
        }
    }
}

impl UCDF {
    /// Add a connection parameter
    pub fn add_connection(&mut self, key: &str, value: &str) -> &mut Self {
        self.connection.insert(key, value);
        self
    }

    /// Fluent API for adding a connection parameter
    pub fn with_connection(mut self, key: &str, value: &str) -> Self {
        self.connection.insert(key, value);
        self
    }

    /// Add fields structure
    pub fn add_fields(&mut self, fields: Vec<Field>) -> &mut Self {
        self.structure
            .insert("fields".to_string(), StructureData::Fields(fields));
        self
    }

    /// Fluent API for adding fields structure
    pub fn with_fields(mut self, fields: Vec<Field>) -> Self {
        self.add_fields(fields);
        self
    }

    /// Add endpoints structure
    pub fn add_endpoints(&mut self, endpoints: Vec<Endpoint>) -> &mut Self {
        self.structure
            .insert("endpoints".to_string(), StructureData::Endpoints(endpoints));
        self
    }

    /// Fluent API for adding endpoints structure
    pub fn with_endpoints(mut self, endpoints: Vec<Endpoint>) -> Self {
        self.add_endpoints(endpoints);
        self
    }

    /// Add format structure
    pub fn add_format(&mut self, format: &str) -> &mut Self {
        self.structure.insert(
            "format".to_string(),
            StructureData::Format(format.to_string()),
        );
        self
    }

    /// Fluent API for adding format structure
    pub fn with_format(mut self, format: &str) -> Self {
        self.add_format(format);
        self
    }

    /// Add custom structure
    pub fn add_custom_structure(&mut self, key: &str, value: &str) -> &mut Self {
        self.structure.insert(
            key.to_string(),
            StructureData::Custom(key.to_string(), value.to_string()),
        );
        self
    }

    /// Fluent API for adding custom structure
    pub fn with_custom_structure(mut self, key: &str, value: &str) -> Self {
        self.add_custom_structure(key, value);
        self
    }

    /// Set access mode
    pub fn set_access_mode(&mut self, mode: AccessMode) -> &mut Self {
        self.access_mode = Some(mode);
        self
    }

    /// Fluent API for setting access mode
    pub fn with_access_mode(mut self, mode: AccessMode) -> Self {
        self.set_access_mode(mode);
        self
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) -> &mut Self {
        self.metadata.insert(key, value);
        self
    }

    /// Fluent API for adding metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.add_metadata(key, value);
        self
    }

    /// Parse a string containing fields
    pub fn parse_fields(fields_str: &str) -> Result<Vec<Field>> {
        let mut fields = Vec::new();
        for field_str in fields_str.split(',') {
            fields.push(Field::from_str(field_str)?);
        }
        Ok(fields)
    }

    /// Parse a string containing endpoints
    pub fn parse_endpoints(endpoints_str: &str) -> Result<Vec<Endpoint>> {
        let mut endpoints = Vec::new();
        for endpoint_str in endpoints_str.split(',') {
            endpoints.push(Endpoint::from_str(endpoint_str)?);
        }
        Ok(endpoints)
    }

    /// Convert the UCDF structure to a string
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();

        // Type section
        parts.push(format!("t={}", self.source_type));

        // Connection parameters
        for (key, value) in self.connection.iter() {
            let formatted_value = if value.contains(';')
                || value.contains('=')
                || value.contains(',')
                || value.contains(':')
            {
                format!("\"{}\"", value)
            } else {
                value.clone()
            };
            parts.push(format!("c.{}={}", key, formatted_value));
        }

        // Structure sections
        for (key, value) in &self.structure {
            match value {
                StructureData::Fields(fields) => {
                    let fields_str = fields
                        .iter()
                        .map(|field| field.to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    parts.push(format!("s.{}={}", key, fields_str));
                }
                StructureData::Endpoints(endpoints) => {
                    let endpoints_str = endpoints
                        .iter()
                        .map(|endpoint| endpoint.to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    parts.push(format!("s.{}={}", key, endpoints_str));
                }
                StructureData::Format(format) => {
                    parts.push(format!("s.{}={}", key, format));
                }
                StructureData::Custom(_, custom_value) => {
                    parts.push(format!("s.{}={}", key, custom_value));
                }
            }
        }

        // Access mode
        if let Some(access_mode) = &self.access_mode {
            parts.push(format!("a={}", access_mode));
        }

        // Metadata
        for (key, value) in self.metadata.iter() {
            let formatted_value = if value.contains(';')
                || value.contains('=')
                || value.contains(',')
                || value.contains(':')
            {
                format!("\"{}\"", value)
            } else {
                value.clone()
            };
            parts.push(format!("m.{}={}", key, formatted_value));
        }

        parts.join(";")
    }
}
