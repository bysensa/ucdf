use std::collections::HashMap;
use std::path::Display;
use std::str::FromStr;
use ucdf::{
    parse, AccessMode, ConnectionParams, DataType, DataValue, Endpoint, Error, Field, Metadata,
    Parser, Result, SourceType, StructureData, UCDF,
};

fn main() -> Result<()> {
    // Example 1: Advanced parsing with options
    println!("Example 1: Advanced parsing with options");
    let ucdf_str =
        "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str,email:str;a=r;m.desc=User data";

    // Use the custom parser with options
    let parser = Parser::new();

    let ucdf = parser.parse(ucdf_str)?;
    println!("Parsed UCDF: {}\n", ucdf.to_string());

    // Example 2: Working with nested data
    println!("Example 2: Working with nested data");
    let nested_ucdf_str = "t=db.postgresql;c.host=db.prod;c.port=5432;c.db=users;c.user=admin;c.password=\"p@ssw0rd;with;semicolons\";s.fields=id:int,profile:json;a=rw;m.desc=\"PostgreSQL database with JSON fields\"";

    let nested_ucdf = parse(nested_ucdf_str)?;

    // Extract specific connection parameters
    println!("Database connection info:");
    println!(
        "  Host: {}",
        nested_ucdf
            .connection
            .get("host")
            .unwrap_or(&"localhost".to_string())
    );
    println!(
        "  Port: {}",
        nested_ucdf
            .connection
            .get("port")
            .unwrap_or(&"5432".to_string())
    );
    println!(
        "  Database: {}",
        nested_ucdf.connection.get("db").unwrap_or(&"".to_string())
    );
    println!(
        "  Username: {}",
        nested_ucdf
            .connection
            .get("user")
            .unwrap_or(&"".to_string())
    );

    // Handle password securely
    if let Some(password) = nested_ucdf.connection.get("password") {
        println!("  Password: {}", "*".repeat(password.len()));
    }

    // Extract and handle fields
    if let Some(StructureData::Fields(fields)) = nested_ucdf.structure.get("fields") {
        for field in fields {
            println!("  Field: {} ({})", field.name, field.dtype);

            // Special handling for JSON fields
            if field.dtype == "json" {
                println!("    This is a JSON field and would require special handling");
            }
        }
    }
    println!();

    // Example 3: Building complex structures manually
    println!("Example 3: Building complex structures manually");

    // Create a REST API UCDF
    let mut connection = ConnectionParams::new();
    connection.insert("url", "https://api.example.com/v2");
    connection.insert("auth.type", "oauth2");
    connection.insert("auth.token", "abc123xyz789");

    // Create structure with endpoints
    let endpoints = vec![
        Endpoint::new("/users".to_string(), "GET".to_string()),
        Endpoint::new("/users/{id}".to_string(), "GET".to_string()),
        Endpoint::new("/users".to_string(), "POST".to_string()),
        Endpoint::new("/users/{id}".to_string(), "PUT".to_string()),
        Endpoint::new("/users/{id}".to_string(), "DELETE".to_string()),
    ];

    let mut structure = HashMap::new();
    structure.insert("endpoints".to_string(), StructureData::Endpoints(endpoints));

    // Add format information
    structure.insert(
        "format".to_string(),
        StructureData::Format("json".to_string()),
    );

    // Add custom structure information
    structure.insert(
        "version".to_string(),
        StructureData::Custom("version".to_string(), "2.0".to_string()),
    );

    // Create metadata
    let mut metadata = Metadata::new();
    metadata.insert("owner", "api-team");
    metadata.insert("env", "production");
    metadata.insert("rate_limit", "100/minute");
    metadata.insert("docs", "https://docs.example.com/api/v2");

    // Create the complete UCDF
    let api_ucdf = UCDF::builder()
        .source_type(SourceType::new("api".to_string(), Some("rest".to_string())))
        .connection(connection)
        .structure(structure)
        .access_mode(AccessMode::ReadWrite)
        .metadata(metadata)
        .build();

    println!("API UCDF: {}\n", api_ucdf.to_string());

    // Example 4: Error handling and validation
    println!("Example 4: Error handling and validation");

    // Handle invalid UCDF string
    let invalid_ucdf = "t=file.csv;invalid_section;c.path=/data/users.csv";
    match parse(invalid_ucdf) {
        Ok(_) => println!("Successfully parsed (unexpected)"),
        Err(e) => println!("Error parsing invalid UCDF: {}", e),
    }

    // Handle missing type section
    let missing_type = "c.path=/data/users.csv;s.fields=id:int,name:str";
    match Parser::new().parse(missing_type) {
        Ok(_) => println!("Successfully parsed (unexpected)"),
        Err(e) => println!("Error parsing UCDF with missing type: {}", e),
    }

    // Error handling with detailed error information
    fn parse_with_detailed_error(input: &str) -> std::result::Result<(), String> {
        match parse(input) {
            Ok(ucdf) => {
                println!("Successfully parsed: {}", ucdf.to_string());
                Ok(())
            }
            Err(Error::MissingTypeSection) => {
                Err("Type section (t=...) is required but missing".to_string())
            }
            Err(Error::InvalidSectionFormat(section)) => {
                Err(format!("Section has invalid format: {}", section))
            }
            Err(Error::InvalidSourceType(source_type)) => {
                Err(format!("Invalid source type format: {}", source_type))
            }
            Err(e) => Err(format!("Other error: {}", e)),
        }
    }

    let result = parse_with_detailed_error("invalid;;;format");
    println!("Detailed error: {:?}\n", result.err().unwrap_or_default());

    // Example 5: Converting between different formats
    println!("Example 5: Converting between different formats");

    // Convert UCDF to connection string for database
    let db_ucdf_str = "t=db.postgresql;c.host=localhost;c.port=5432;c.db=myapp;c.user=postgres;c.password=secret;a=rw";
    let db_ucdf = parse(db_ucdf_str)?;

    // Convert to PostgreSQL connection string
    let pg_conn_string = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_ucdf
            .connection
            .get("user")
            .unwrap_or(&"postgres".to_string()),
        db_ucdf
            .connection
            .get("password")
            .unwrap_or(&"".to_string()),
        db_ucdf
            .connection
            .get("host")
            .unwrap_or(&"localhost".to_string()),
        db_ucdf
            .connection
            .get("port")
            .unwrap_or(&"5432".to_string()),
        db_ucdf
            .connection
            .get("db")
            .unwrap_or(&"postgres".to_string()),
    );
    println!("PostgreSQL connection string: {}", pg_conn_string);

    // Convert UCDF to URL for REST API
    let api_ucdf_str =
        "t=api.rest;c.url=https://api.example.com;c.path=/users;c.params=limit=100,offset=0;a=r";
    let api_ucdf = parse(api_ucdf_str)?;

    let base_url = api_ucdf
        .connection
        .get("url")
        .map(Into::into)
        .unwrap_or("".to_string());
    let path = api_ucdf
        .connection
        .get("path")
        .map(Into::into)
        .unwrap_or("".to_string());
    let params = api_ucdf
        .connection
        .get("params")
        .map(Into::into)
        .unwrap_or("".to_string());

    let api_url = if params.is_empty() {
        format!("{}{}", base_url, path)
    } else {
        format!("{}{}?{}", base_url, path, params.replace(',', "&"))
    };

    println!("API URL: {}\n", api_url);

    Ok(())
}
