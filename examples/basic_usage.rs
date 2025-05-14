use std::str::FromStr;
use ucdf::{parse, AccessMode, DataValue, Endpoint, Field, SourceType, StructureData, UCDF};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Parse a UCDF string
    println!("Example 1: Parsing a UCDF string");
    let ucdf_str =
        "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str,email:str;a=r;m.desc=User data";
    let ucdf = parse(ucdf_str)?;

    // Access the parsed data
    println!("Source type: {}", ucdf.source_type);
    println!(
        "Path: {}",
        ucdf.connection.get("path").unwrap_or(&"N/A".to_string())
    );
    println!("Access mode: {:?}", ucdf.access_mode);
    println!(
        "Description: {}",
        ucdf.metadata.get("desc").unwrap_or(&"N/A".to_string())
    );

    // Access the fields
    if let Some(StructureData::Fields(fields)) = ucdf.structure.get("fields") {
        println!("Fields:");
        for field in fields {
            println!("  - {} ({})", field.name, field.dtype);
        }
    }

    println!("\n---\n");

    // Example 2: Create a UCDF structure programmatically
    println!("Example 2: Creating a UCDF structure programmatically");
    let source_type = SourceType::new("api".to_string(), Some("rest".to_string()));
    let mut ucdf = UCDF::with_source_type(source_type);

    // Add connection parameters
    ucdf.add_connection("url", "https://api.example.com")
        .add_connection("auth.type", "bearer")
        .add_connection("auth.token", "xyz");

    // Add endpoints
    let endpoints = vec![
        Endpoint::new("/users".to_string(), "GET".to_string()),
        Endpoint::new("/orders".to_string(), "POST".to_string()),
    ];
    ucdf.add_endpoints(endpoints);

    // Set access mode
    ucdf.set_access_mode(AccessMode::ReadWrite);

    // Add metadata
    ucdf.add_metadata("owner", "admin")
        .add_metadata("tags", "production,critical");

    // Convert back to string
    let ucdf_str = ucdf.to_string();
    println!("UCDF string: {}", ucdf_str);

    println!("\n---\n");

    // Example 3: Using the fluent API to create a Kafka UCDF
    println!("Example 3: Using the fluent API to create a Kafka UCDF");
    let kafka_ucdf = UCDF::with_source_type(SourceType::new(
        "stream".to_string(),
        Some("kafka".to_string()),
    ))
    .with_connection("brokers", "broker1:9092,broker2:9092")
    .with_connection("topic", "events")
    .with_format("json")
    .with_fields(vec![
        Field::new("event_id".to_string(), "str".to_string(), None),
        Field::new("payload".to_string(), "json".to_string(), None),
    ])
    .with_access_mode(AccessMode::Read)
    .with_metadata("desc", "Kafka event stream");

    println!("Kafka UCDF: {}", kafka_ucdf.to_string());

    println!("\n---\n");

    // Example 4: Working with values and data types
    println!("Example 4: Working with values and data types");
    let field_with_value = Field::new(
        "temperature".to_string(),
        "float".to_string(),
        Some(DataValue::Float(25.5)),
    );

    println!(
        "Field: {} = {}",
        field_with_value.name,
        match &field_with_value.value {
            Some(value) => format!("{}", value),
            None => "N/A".to_string(),
        }
    );

    // Parse a value using the DataValue API
    let temp_str = "36.6";
    let parsed_value = DataValue::parse(temp_str, "float")?;
    println!(
        "Parsed value: {} ({})",
        parsed_value,
        parsed_value.type_name()
    );

    Ok(())
}
