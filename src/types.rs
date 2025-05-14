use std::fmt;
use std::str::FromStr;

use bon::bon;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Represents a field value with type information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataValue {
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// JSON value as a string
    Json(String),
    /// Date value in ISO 8601 format
    Date(String),
    /// DateTime value in ISO 8601 format
    DateTime(String),
    /// Custom data type with value
    Custom(String, String),
}

impl fmt::Display for DataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataValue::String(s) => write!(f, "{}", s),
            DataValue::Integer(i) => write!(f, "{}", i),
            DataValue::Float(fl) => write!(f, "{}", fl),
            DataValue::Boolean(b) => write!(f, "{}", b),
            DataValue::Json(j) => write!(f, "{}", j),
            DataValue::Date(d) => write!(f, "{}", d),
            DataValue::DateTime(dt) => write!(f, "{}", dt),
            DataValue::Custom(_, val) => write!(f, "{}", val),
        }
    }
}

impl DataValue {
    /// Get the type name of this data value
    pub fn type_name(&self) -> &'static str {
        match self {
            DataValue::String(_) => "str",
            DataValue::Integer(_) => "int",
            DataValue::Float(_) => "float",
            DataValue::Boolean(_) => "bool",
            DataValue::Json(_) => "json",
            DataValue::Date(_) => "date",
            DataValue::DateTime(_) => "datetime",
            DataValue::Custom(custom_type, _) => {
                // Return a static string based on the custom type
                match custom_type.as_str() {
                    // Add cases for known custom types if needed
                    _ => "custom", // Default fallback for custom types
                }
            }
        }
    }

    /// Parse a string value into a DataValue based on the specified type
    pub fn parse(value: &str, dtype: &str) -> Result<Self> {
        match dtype {
            "str" => Ok(DataValue::String(value.to_string())),
            "int" => match value.parse::<i64>() {
                Ok(i) => Ok(DataValue::Integer(i)),
                Err(_) => Err(Error::ParseError(format!(
                    "Failed to parse '{}' as integer",
                    value
                ))),
            },
            "float" => match value.parse::<f64>() {
                Ok(f) => Ok(DataValue::Float(f)),
                Err(_) => Err(Error::ParseError(format!(
                    "Failed to parse '{}' as float",
                    value
                ))),
            },
            "bool" => match value.parse::<bool>() {
                Ok(b) => Ok(DataValue::Boolean(b)),
                Err(_) => Err(Error::ParseError(format!(
                    "Failed to parse '{}' as boolean",
                    value
                ))),
            },
            "json" => Ok(DataValue::Json(value.to_string())),
            "date" => Ok(DataValue::Date(value.to_string())),
            "datetime" => Ok(DataValue::DateTime(value.to_string())),
            _ => Ok(DataValue::Custom(dtype.to_string(), value.to_string())),
        }
    }
}

/// Field definition with name and type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub dtype: String,
    pub value: Option<DataValue>,
}

#[bon]
impl Field {
    #[builder]
    pub fn builder(name: String, dtype: String, value: Option<DataValue>) -> Self {
        Self { name, dtype, value }
    }

    pub fn new(name: String, dtype: String, value: Option<DataValue>) -> Self {
        Self { name, dtype, value }
    }
}

impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidFieldFormat(s.to_string()));
        }

        Ok(Field {
            name: parts[0].to_string(),
            dtype: parts[1].to_string(),
            value: None,
        })
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.dtype)
    }
}

/// Endpoint definition with path and method
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub method: String,
}

#[bon]
impl Endpoint {
    #[builder]
    pub fn builder(path: String, method: String) -> Self {
        Self { path, method }
    }

    pub fn new(path: String, method: String) -> Self {
        Self { path, method }
    }
}

impl FromStr for Endpoint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidEndpointFormat(s.to_string()));
        }

        Ok(Endpoint {
            path: parts[0].to_string(),
            method: parts[1].to_string(),
        })
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.path, self.method)
    }
}
