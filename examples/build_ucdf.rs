use std::collections::HashMap;
use ucdf::{
    AccessMode, ConnectionParams, Endpoint, Field, Metadata, SourceType, StructureData, UCDF,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building UCDF Structures from Scratch\n");

    // Example 1: Basic CSV file
    println!("Example 1: CSV File");
    let csv_ucdf = build_csv_example();
    println!("CSV UCDF: {}\n", csv_ucdf.to_string());

    // Example 2: Database connection
    println!("Example 2: Database Connection");
    let db_ucdf = build_database_example();
    println!("Database UCDF: {}\n", db_ucdf.to_string());

    // Example 3: REST API
    println!("Example 3: REST API");
    let api_ucdf = build_api_example();
    println!("API UCDF: {}\n", api_ucdf.to_string());

    // Example 4: Kafka Stream
    println!("Example 4: Kafka Stream");
    let stream_ucdf = build_stream_example();
    println!("Stream UCDF: {}\n", stream_ucdf.to_string());

    // Example 5: Custom data source
    println!("Example 5: Custom Data Source");
    let custom_ucdf = build_custom_example();
    println!("Custom UCDF: {}\n", custom_ucdf.to_string());

    Ok(())
}

// Build a CSV file UCDF example
fn build_csv_example() -> UCDF {
    // Create source type
    let source_type = SourceType::new("file".to_string(), Some("csv".to_string()));

    // Create connection parameters
    let mut connection = ConnectionParams::new();
    connection.insert("path", "/data/reports/sales_2023.csv");
    connection.insert("encoding", "utf-8");

    // Create fields
    let fields = vec![
        Field::new("id".to_string(), "int".to_string(), None),
        Field::new("date".to_string(), "date".to_string(), None),
        Field::new("customer_id".to_string(), "int".to_string(), None),
        Field::new("product_id".to_string(), "int".to_string(), None),
        Field::new("quantity".to_string(), "int".to_string(), None),
        Field::new("price".to_string(), "float".to_string(), None),
        Field::new("total".to_string(), "float".to_string(), None),
    ];

    // Create structure
    let mut structure = HashMap::new();
    structure.insert("fields".to_string(), StructureData::Fields(fields));

    // Create metadata
    let mut metadata = Metadata::new();
    metadata.insert("desc", "Sales data for 2023");
    metadata.insert("owner", "sales-department");
    metadata.insert("updated", "2023-12-31");

    UCDF::builder()
        .structure(structure)
        .source_type(source_type)
        .connection(connection)
        .metadata(metadata)
        .access_mode(AccessMode::Read)
        .build()
    // Create UCDF
}

// Build a database UCDF example
fn build_database_example() -> UCDF {
    // Create source type
    let source_type = SourceType::new("db".to_string(), Some("postgresql".to_string()));

    // Create connection parameters
    let mut connection = ConnectionParams::new();
    connection.insert("host", "db.example.com");
    connection.insert("port", "5432");
    connection.insert("db", "customers");
    connection.insert("user", "app_user");
    connection.insert("password", "secure_password");
    connection.insert("sslmode", "require");

    // Create fields
    let fields = vec![
        Field::new("id".to_string(), "int".to_string(), None),
        Field::new("name".to_string(), "str".to_string(), None),
        Field::new("email".to_string(), "str".to_string(), None),
        Field::new("created_at".to_string(), "datetime".to_string(), None),
        Field::new("status".to_string(), "str".to_string(), None),
    ];

    // Create structure
    let mut structure = HashMap::new();
    structure.insert("fields".to_string(), StructureData::Fields(fields));
    structure.insert(
        "table".to_string(),
        StructureData::Custom("table".to_string(), "customers".to_string()),
    );

    // Create metadata
    let mut metadata = Metadata::new();
    metadata.insert("desc", "Customer database");
    metadata.insert("env", "production");

    // Create UCDF
    UCDF::builder()
        .structure(structure)
        .source_type(source_type)
        .connection(connection)
        .metadata(metadata)
        .access_mode(AccessMode::ReadWrite)
        .build()
}

// Build a REST API UCDF example
fn build_api_example() -> UCDF {
    // Create source type
    let source_type = SourceType::new("api".to_string(), Some("rest".to_string()));

    // Create connection parameters
    let mut connection = ConnectionParams::new();
    connection.insert("url", "https://api.example.com/v2");
    connection.insert("auth.type", "bearer");
    connection.insert("auth.token", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...");
    connection.insert("timeout", "30");

    // Create endpoints
    let endpoints = vec![
        Endpoint::new("/users".to_string(), "GET".to_string()),
        Endpoint::new("/users/{id}".to_string(), "GET".to_string()),
        Endpoint::new("/users".to_string(), "POST".to_string()),
        Endpoint::new("/users/{id}".to_string(), "PUT".to_string()),
        Endpoint::new("/users/{id}".to_string(), "DELETE".to_string()),
    ];

    // Create structure
    let mut structure = HashMap::new();
    structure.insert("endpoints".to_string(), StructureData::Endpoints(endpoints));
    structure.insert(
        "format".to_string(),
        StructureData::Format("json".to_string()),
    );
    structure.insert(
        "version".to_string(),
        StructureData::Custom("version".to_string(), "2.0".to_string()),
    );

    // Create metadata
    let mut metadata = Metadata::new();
    metadata.insert("desc", "User management API");
    metadata.insert("docs", "https://docs.example.com/api/v2/users");

    // Create UCDF
    UCDF::builder()
        .structure(structure)
        .source_type(source_type)
        .connection(connection)
        .metadata(metadata)
        .access_mode(AccessMode::ReadWrite)
        .build()
}

// Build a Kafka stream UCDF example
fn build_stream_example() -> UCDF {
    // Create source type
    let source_type = SourceType::new("stream".to_string(), Some("kafka".to_string()));

    // Create connection parameters
    let mut connection = ConnectionParams::new();
    connection.insert("brokers", "kafka1.example.com:9092,kafka2.example.com:9092");
    connection.insert("topic", "user-events");
    connection.insert("group_id", "consumer-group-1");
    connection.insert("auto.offset.reset", "earliest");

    // Create fields
    let fields = vec![
        Field::new("event_id".to_string(), "str".to_string(), None),
        Field::new("user_id".to_string(), "int".to_string(), None),
        Field::new("event_type".to_string(), "str".to_string(), None),
        Field::new("timestamp".to_string(), "datetime".to_string(), None),
        Field::new("data".to_string(), "json".to_string(), None),
    ];

    // Create structure
    let mut structure = HashMap::new();
    structure.insert("fields".to_string(), StructureData::Fields(fields));
    structure.insert(
        "format".to_string(),
        StructureData::Format("json".to_string()),
    );

    // Create metadata
    let mut metadata = Metadata::new();
    metadata.insert("desc", "User event stream");
    metadata.insert("retention", "7d");
    metadata.insert("partitions", "12");

    // Create UCDF
    UCDF::builder()
        .structure(structure)
        .source_type(source_type)
        .connection(connection)
        .metadata(metadata)
        .access_mode(AccessMode::ReadWrite)
        .build()
}

// Build a custom data source UCDF example
fn build_custom_example() -> UCDF {
    // Create source type
    let source_type = SourceType::new("custom".to_string(), Some("iot".to_string()));

    // Create connection parameters
    let mut connection = ConnectionParams::new();
    connection.insert("protocol", "mqtt");
    connection.insert("host", "iot.example.com");
    connection.insert("port", "1883");
    connection.insert("client_id", "sensor-reader-1");
    connection.insert("topic", "sensors/temperature/#");
    connection.insert("qos", "1");

    // Create fields
    let fields = vec![
        Field::new("device_id".to_string(), "str".to_string(), None),
        Field::new("sensor_type".to_string(), "str".to_string(), None),
        Field::new("value".to_string(), "float".to_string(), None),
        Field::new("unit".to_string(), "str".to_string(), None),
        Field::new("timestamp".to_string(), "datetime".to_string(), None),
        Field::new("battery".to_string(), "float".to_string(), None),
        Field::new("rssi".to_string(), "int".to_string(), None),
    ];

    // Create structure
    let mut structure = HashMap::new();
    structure.insert("fields".to_string(), StructureData::Fields(fields));
    structure.insert(
        "format".to_string(),
        StructureData::Format("json".to_string()),
    );
    structure.insert(
        "frequency".to_string(),
        StructureData::Custom("frequency".to_string(), "60s".to_string()),
    );

    // Create metadata
    let mut metadata = Metadata::new();
    metadata.insert("desc", "IoT temperature sensors");
    metadata.insert("location", "Building A");
    metadata.insert("deployment", "2023-06-01");

    // Create UCDF
    UCDF::builder()
        .structure(structure)
        .source_type(source_type)
        .connection(connection)
        .metadata(metadata)
        .access_mode(AccessMode::ReadWrite)
        .build()
}
