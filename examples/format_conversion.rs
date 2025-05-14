use std::collections::HashMap;

use ucdf::{
    parse, AccessMode, ConnectionParams, DataValue, Field, SourceType, StructureData, UCDF,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("UCDF Format Conversion Examples");
    println!("==============================\n");

    // Example 1: Convert UCDF to Database Connection Strings
    println!("Example 1: UCDF to Database Connection Strings");

    // PostgreSQL UCDF
    let pg_ucdf_str = "t=db.postgresql;c.host=localhost;c.port=5432;c.db=mydatabase;c.user=postgres;c.password=s3cur3p@ss;a=rw";
    let pg_ucdf = parse(pg_ucdf_str)?;

    // Convert to PostgreSQL connection string
    let pg_conn_string = format!(
        "postgresql://{}:{}@{}:{}/{}",
        pg_ucdf
            .connection
            .get("user")
            .unwrap_or(&"postgres".to_string()),
        pg_ucdf
            .connection
            .get("password")
            .unwrap_or(&"".to_string()),
        pg_ucdf
            .connection
            .get("host")
            .unwrap_or(&"localhost".to_string()),
        pg_ucdf
            .connection
            .get("port")
            .unwrap_or(&"5432".to_string()),
        pg_ucdf
            .connection
            .get("db")
            .unwrap_or(&"postgres".to_string()),
    );
    println!("UCDF: {}", pg_ucdf_str);
    println!("PostgreSQL Connection String: {}\n", pg_conn_string);

    // MySQL UCDF
    let mysql_ucdf_str = "t=db.mysql;c.host=db.example.com;c.port=3306;c.db=app_data;c.user=dbuser;c.password=dbpass;c.params=charset=utf8mb4;a=r";
    let mysql_ucdf = parse(mysql_ucdf_str)?;

    // Convert to MySQL connection string
    let mysql_conn_string = format!(
        "mysql://{}:{}@{}:{}/{}?{}",
        mysql_ucdf
            .connection
            .get("user")
            .unwrap_or(&"root".to_string()),
        mysql_ucdf
            .connection
            .get("password")
            .unwrap_or(&"".to_string()),
        mysql_ucdf
            .connection
            .get("host")
            .unwrap_or(&"localhost".to_string()),
        mysql_ucdf
            .connection
            .get("port")
            .unwrap_or(&"3306".to_string()),
        mysql_ucdf.connection.get("db").unwrap_or(&"".to_string()),
        mysql_ucdf
            .connection
            .get("params")
            .unwrap_or(&"".to_string()),
    );
    println!("UCDF: {}", mysql_ucdf_str);
    println!("MySQL Connection String: {}\n", mysql_conn_string);

    // MongoDB UCDF
    let mongo_ucdf_str = "t=db.mongodb;c.uri=mongodb://user:pass@mongo.example.com:27017;c.db=analytics;s.fields=_id:str,timestamp:datetime,value:float;a=rw";
    let mongo_ucdf = parse(mongo_ucdf_str)?;

    println!("UCDF: {}", mongo_ucdf_str);
    println!(
        "MongoDB Connection: {}/{}\n",
        mongo_ucdf
            .connection
            .get("uri")
            .unwrap_or(&"mongodb://localhost:27017".to_string()),
        mongo_ucdf
            .connection
            .get("db")
            .unwrap_or(&"test".to_string())
    );

    // Example 2: Convert UCDF to API Request Information
    println!("\nExample 2: UCDF to API Request Information");

    let api_ucdf_str = "t=api.rest;c.url=https://api.example.com;c.path=/users;c.params=limit=100,offset=0;c.auth.type=bearer;c.auth.token=xyz123;a=r";
    let api_ucdf = parse(api_ucdf_str)?;

    // Build full URL with parameters
    let base_url = api_ucdf.connection.get("url").unwrap();
    let path = api_ucdf.connection.get("path").unwrap();
    let params = api_ucdf.connection.get("params").unwrap();

    let api_url = if params.is_empty() {
        format!("{}{}", base_url, path)
    } else {
        format!("{}{}?{}", base_url, path, params.replace(',', "&"))
    };

    // Build headers
    let mut headers = HashMap::new();
    if let (Some(auth_type), Some(auth_token)) = (
        api_ucdf.connection.get("auth.type"),
        api_ucdf.connection.get("auth.token"),
    ) {
        if auth_type == "bearer" {
            headers.insert(
                "Authorization".to_string(),
                format!("Bearer {}", auth_token),
            );
        } else if auth_type == "basic" {
            headers.insert("Authorization".to_string(), format!("Basic {}", auth_token));
        }
    }

    println!("UCDF: {}", api_ucdf_str);
    println!("API URL: {}", api_url);
    println!("Headers: {:?}\n", headers);

    // Example 3: Convert UCDF to File Path and Format Information
    println!("\nExample 3: UCDF to File Path and Format Information");

    let file_ucdf_str = "t=file.csv;c.path=/data/reports/daily_stats.csv;c.encoding=utf-8;s.fields=date:date,product:str,sales:int,revenue:float;a=r;m.desc=Daily sales report";
    let file_ucdf = parse(file_ucdf_str)?;

    // Get file information
    let file_path = file_ucdf
        .connection
        .get("path")
        .map(Into::into)
        .unwrap_or("".to_string());
    let encoding = file_ucdf
        .connection
        .get("encoding")
        .map(Into::into)
        .unwrap_or("utf-8".to_string());

    // Get schema information
    let mut field_types = Vec::new();
    if let Some(StructureData::Fields(fields)) = file_ucdf.structure.get("fields") {
        for field in fields {
            field_types.push(format!("{}: {}", field.name, field.dtype));
        }
    }

    println!("UCDF: {}", file_ucdf_str);
    println!("File Path: {}", file_path);
    println!("Encoding: {}", encoding);
    println!("Schema: {}\n", field_types.join(", "));

    // Example 4: Convert UCDF to Message Queue Configuration
    println!("\nExample 4: UCDF to Message Queue Configuration");

    let kafka_ucdf_str = "t=stream.kafka;c.brokers=broker1:9092,broker2:9092;c.topic=events;c.group_id=consumer_group_1;s.format=json;s.fields=event_id:str,timestamp:datetime,payload:json;a=rw";
    let kafka_ucdf = parse(kafka_ucdf_str)?;

    // Extract connection details
    let brokers = kafka_ucdf
        .connection
        .get("brokers")
        .map(Into::into)
        .unwrap_or("".to_string());
    let topic = kafka_ucdf
        .connection
        .get("topic")
        .map(Into::into)
        .unwrap_or("".to_string());
    let group_id = kafka_ucdf
        .connection
        .get("group_id")
        .map(Into::into)
        .unwrap_or("".to_string());

    // Format as consumer configuration
    let consumer_config = format!(
        r#"{{
  "bootstrap.servers": "{}",
  "group.id": "{}",
  "auto.offset.reset": "earliest",
  "enable.auto.commit": true
}}"#,
        brokers, group_id
    );

    println!("UCDF: {}", kafka_ucdf_str);
    println!("Kafka Topic: {}", topic);
    println!("Consumer Configuration: \n{}\n", consumer_config);

    // Example 5: Convert from other formats to UCDF
    println!("\nExample 5: Convert from other formats to UCDF");

    // Convert a JDBC URL to UCDF
    let jdbc_url = "jdbc:postgresql://dbserver:5432/inventory?user=admin&password=secret";

    // Parse JDBC URL
    let ucdf_from_jdbc = jdbc_to_ucdf(jdbc_url)?;
    println!("JDBC URL: {}", jdbc_url);
    println!("As UCDF: {}\n", ucdf_from_jdbc.to_string());

    // Convert a MongoDB connection string to UCDF
    let mongo_uri = "mongodb://username:p%40ssw0rd@mongodb0.example.com:27017,mongodb1.example.com:27017/admin?replicaSet=myRepl&w=majority&retryWrites=true";

    // Parse MongoDB URI
    let ucdf_from_mongo = mongodb_uri_to_ucdf(mongo_uri)?;
    println!("MongoDB URI: {}", mongo_uri);
    println!("As UCDF: {}", ucdf_from_mongo.to_string());

    Ok(())
}

// Convert JDBC URL to UCDF
fn jdbc_to_ucdf(jdbc_url: &str) -> Result<UCDF, Box<dyn std::error::Error>> {
    // Basic parsing of JDBC URL
    // Format: jdbc:<engine>://<host>:<port>/<database>?param1=value1&param2=value2

    let parts: Vec<&str> = jdbc_url.splitn(2, "://").collect();
    if parts.len() != 2 {
        return Err("Invalid JDBC URL format".into());
    }

    let engine_part = parts[0];
    let rest = parts[1];

    let engine_parts: Vec<&str> = engine_part.split(':').collect();
    if engine_parts.len() < 2 {
        return Err("Invalid JDBC engine format".into());
    }

    let engine = engine_parts[1];

    // Parse host, port, database and params
    let mut host_db_parts = rest.splitn(2, '?');
    let host_db = host_db_parts.next().unwrap_or("");
    let params_str = host_db_parts.next().unwrap_or("");

    let mut host_db_split = host_db.splitn(2, '/');
    let host_port = host_db_split.next().unwrap_or("");
    let database = host_db_split.next().unwrap_or("");

    let mut host_port_split = host_port.splitn(2, ':');
    let host = host_port_split.next().unwrap_or("");
    let port = host_port_split.next().unwrap_or("");

    // Create UCDF
    let source_type = SourceType::new("db".to_string(), Some(engine.to_string()));

    let mut ucdf = UCDF::with_source_type(source_type);

    // Add connection parameters
    ucdf.add_connection("host", host);
    if !port.is_empty() {
        ucdf.add_connection("port", port);
    }
    if !database.is_empty() {
        ucdf.add_connection("db", database);
    }

    // Parse query parameters
    if !params_str.is_empty() {
        for param in params_str.split('&') {
            let kv: Vec<&str> = param.splitn(2, '=').collect();
            if kv.len() == 2 {
                let key = kv[0];
                let value = kv[1];

                // Special handling for common parameters
                match key {
                    "user" => ucdf.add_connection("user", value),
                    "password" => ucdf.add_connection("password", value),
                    _ => ucdf.add_connection(&format!("params.{}", key), value),
                };
            }
        }
    }

    // Set access mode (assume read-write for database connections)
    ucdf.set_access_mode(AccessMode::ReadWrite);

    Ok(ucdf)
}

// Convert MongoDB URI to UCDF
fn mongodb_uri_to_ucdf(mongo_uri: &str) -> Result<UCDF, Box<dyn std::error::Error>> {
    // Basic parsing of MongoDB URI
    // Format: mongodb://username:password@host1:port1,host2:port2/database?options

    // Create UCDF with MongoDB type
    let source_type = SourceType::new("db".to_string(), Some("mongodb".to_string()));

    let mut ucdf = UCDF::with_source_type(source_type);

    // Store the full URI
    ucdf.add_connection("uri", mongo_uri);

    // Extract database name if present
    if let Some(db_part) = mongo_uri.split('/').nth(3) {
        let db_name = db_part.split('?').next().unwrap_or("");
        if !db_name.is_empty() {
            ucdf.add_connection("db", db_name);
        }
    }

    // Set access mode
    ucdf.set_access_mode(AccessMode::ReadWrite);

    // Add metadata
    ucdf.add_metadata("source", "mongodb_uri");

    Ok(ucdf)
}
