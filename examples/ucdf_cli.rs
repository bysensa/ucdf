use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process;

use ucdf::{parse, AccessMode, DataValue, Field, SourceType, StructureData, UCDF};

const HELP_TEXT: &str = r#"
UCDF CLI - A command-line tool for working with Unified Compact Data Format

Usage:
  ucdf_cli [command] [options]

Commands:
  parse [ucdf_string]           Parse a UCDF string and display its components
  validate [ucdf_string]        Validate a UCDF string without displaying components
  convert [from] [to] [input]   Convert between UCDF and other formats
  generate [type]               Generate a sample UCDF string
  help                          Display this help message

Examples:
  ucdf_cli parse "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str;a=r"
  ucdf_cli validate "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str;a=r"
  ucdf_cli convert jdbc ucdf "jdbc:postgresql://localhost:5432/mydb?user=postgres&password=secret"
  ucdf_cli convert ucdf url "t=api.rest;c.url=https://api.example.com;c.path=/users;c.params=limit=100"
  ucdf_cli generate csv
"#;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No command specified");
        eprintln!("{}", HELP_TEXT);
        process::exit(1);
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: No UCDF string provided");
                process::exit(1);
            }
            parse_command(&args[2]);
        }
        "validate" => {
            if args.len() < 3 {
                eprintln!("Error: No UCDF string provided");
                process::exit(1);
            }
            validate_command(&args[2]);
        }
        "convert" => {
            if args.len() < 5 {
                eprintln!("Error: Conversion requires [from] [to] [input] parameters");
                process::exit(1);
            }
            convert_command(&args[2], &args[3], &args[4]);
        }
        "generate" => {
            if args.len() < 3 {
                eprintln!("Error: No data source type specified");
                process::exit(1);
            }
            generate_command(&args[2]);
        }
        "help" | "--help" | "-h" => {
            println!("{}", HELP_TEXT);
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", args[1]);
            eprintln!("{}", HELP_TEXT);
            process::exit(1);
        }
    }
}

fn parse_command(ucdf_str: &str) {
    match parse(ucdf_str) {
        Ok(ucdf) => {
            println!("Successfully parsed UCDF string");
            println!("------------------------------");

            // Display source type
            println!("Source Type:");
            println!("  Category: {}", ucdf.source_type.category);
            if let Some(subtype) = &ucdf.source_type.subtype {
                println!("  Subtype: {}", subtype);
            }

            // Display connection parameters
            if ucdf.connection.0.len() > 0 {
                println!("\nConnection Parameters:");
                for (key, value) in ucdf.connection.iter() {
                    if key.contains("password") || key.contains("token") {
                        println!("  {}: {}", key, "*".repeat(value.len()));
                    } else {
                        println!("  {}: {}", key, value);
                    }
                }
            }

            // Display structure
            if ucdf.structure.len() > 0 {
                println!("\nStructure:");
                for (key, value) in &ucdf.structure {
                    match value {
                        StructureData::Fields(fields) => {
                            println!("  Fields ({})", key);
                            for field in fields {
                                println!("    {}: {}", field.name, field.dtype);
                            }
                        }
                        StructureData::Endpoints(endpoints) => {
                            println!("  Endpoints ({})", key);
                            for endpoint in endpoints {
                                println!("    {}: {}", endpoint.path, endpoint.method);
                            }
                        }
                        StructureData::Format(format) => {
                            println!("  Format ({}): {}", key, format);
                        }
                        StructureData::Custom(_, custom_value) => {
                            println!("  Custom ({}): {}", key, custom_value);
                        }
                    }
                }
            }

            // Display access mode
            if let Some(access_mode) = &ucdf.access_mode {
                println!("\nAccess Mode:");
                match access_mode {
                    AccessMode::Read => println!("  Read-only (r)"),
                    AccessMode::Write => println!("  Write-only (w)"),
                    AccessMode::ReadWrite => println!("  Read-write (rw)"),
                }
            }

            // Display metadata
            if ucdf.metadata.0.len() > 0 {
                println!("\nMetadata:");
                for (key, value) in ucdf.metadata.iter() {
                    println!("  {}: {}", key, value);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing UCDF string: {}", e);
            process::exit(1);
        }
    }
}

fn validate_command(ucdf_str: &str) {
    match parse(ucdf_str) {
        Ok(_) => {
            println!("Valid UCDF string");
        }
        Err(e) => {
            eprintln!("Invalid UCDF string: {}", e);
            process::exit(1);
        }
    }
}

fn convert_command(from: &str, to: &str, input: &str) {
    match (from, to) {
        ("ucdf", "url") => {
            // Convert UCDF to URL
            match parse(input) {
                Ok(ucdf) => {
                    if ucdf.source_type.category != "api" {
                        eprintln!("Error: Can only convert API UCDF to URL");
                        process::exit(1);
                    }

                    let base_url = ucdf
                        .connection
                        .get("url")
                        .map(Into::into)
                        .unwrap_or("".to_string());
                    let path = ucdf
                        .connection
                        .get("path")
                        .map(Into::into)
                        .unwrap_or("".to_string());
                    let params = ucdf
                        .connection
                        .get("params")
                        .map(Into::into)
                        .unwrap_or("".to_string());

                    let url = if params.is_empty() {
                        format!("{}{}", base_url, path)
                    } else {
                        format!("{}{}?{}", base_url, path, params.replace(',', "&"))
                    };

                    println!("{}", url);
                }
                Err(e) => {
                    eprintln!("Error parsing UCDF string: {}", e);
                    process::exit(1);
                }
            }
        }
        ("ucdf", "jdbc") => {
            // Convert UCDF to JDBC URL
            match parse(input) {
                Ok(ucdf) => {
                    if ucdf.source_type.category != "db" {
                        eprintln!("Error: Can only convert database UCDF to JDBC");
                        process::exit(1);
                    }

                    let db_type = ucdf.source_type.subtype.clone().unwrap_or_default();
                    let host = ucdf
                        .connection
                        .get("host")
                        .map(Into::into)
                        .unwrap_or("localhost".to_string());
                    let port = ucdf
                        .connection
                        .get("port")
                        .map(Into::into)
                        .unwrap_or("".to_string());
                    let db = ucdf
                        .connection
                        .get("db")
                        .map(Into::into)
                        .unwrap_or("".to_string());
                    let user = ucdf
                        .connection
                        .get("user")
                        .map(Into::into)
                        .unwrap_or("".to_string());
                    let password = ucdf
                        .connection
                        .get("password")
                        .map(Into::into)
                        .unwrap_or("".to_string());

                    let port_part = if port.is_empty() {
                        "".into()
                    } else {
                        format!(":{}", port)
                    };
                    let auth_part = if user.is_empty() {
                        "".to_string()
                    } else if password.is_empty() {
                        format!("user={}", user)
                    } else {
                        format!("user={}&password={}", user, password)
                    };

                    let jdbc_url = format!(
                        "jdbc:{}://{}{}{}{}",
                        db_type,
                        host,
                        port_part,
                        if db.is_empty() {
                            "".into()
                        } else {
                            format!("/{}", db)
                        },
                        if auth_part.is_empty() {
                            "".into()
                        } else {
                            format!("?{}", auth_part)
                        }
                    );

                    println!("{}", jdbc_url);
                }
                Err(e) => {
                    eprintln!("Error parsing UCDF string: {}", e);
                    process::exit(1);
                }
            }
        }
        ("jdbc", "ucdf") => {
            // Convert JDBC URL to UCDF
            // Format: jdbc:<engine>://<host>:<port>/<database>?param1=value1&param2=value2
            if !input.starts_with("jdbc:") {
                eprintln!("Error: Invalid JDBC URL format");
                process::exit(1);
            }

            let parts: Vec<&str> = input.splitn(2, "://").collect();
            if parts.len() != 2 {
                eprintln!("Error: Invalid JDBC URL format");
                process::exit(1);
            }

            let engine_part = parts[0];
            let rest = parts[1];

            let engine_parts: Vec<&str> = engine_part.split(':').collect();
            if engine_parts.len() < 2 {
                eprintln!("Error: Invalid JDBC engine format");
                process::exit(1);
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

            println!("{}", ucdf.to_string());
        }
        ("url", "ucdf") => {
            // Convert URL to UCDF
            // Format: <protocol>://<host>[:<port>]/<path>[?<query>]
            if !input.contains("://") {
                eprintln!("Error: Invalid URL format");
                process::exit(1);
            }

            let parts: Vec<&str> = input.splitn(2, "://").collect();
            if parts.len() != 2 {
                eprintln!("Error: Invalid URL format");
                process::exit(1);
            }

            let protocol = parts[0];
            let rest = parts[1];

            // Parse host, path and query
            let rest_parts: Vec<&str> = rest.splitn(2, '/').collect();
            if rest_parts.is_empty() {
                eprintln!("Error: Invalid URL format");
                process::exit(1);
            }

            let host_port = rest_parts[0];

            // Create URL path
            let path_query = if rest_parts.len() > 1 {
                format!("/{}", rest_parts[1])
            } else {
                "".to_string()
            };

            // Split path and query
            let path_query_parts: Vec<&str> = path_query.splitn(2, '?').collect();
            let path = if path_query_parts.is_empty() {
                ""
            } else {
                path_query_parts[0]
            };
            let query = if path_query_parts.len() > 1 {
                path_query_parts[1]
            } else {
                ""
            };

            // Create UCDF
            let source_type = SourceType::new("api".to_string(), Some("rest".to_string()));
            let mut ucdf = UCDF::with_source_type(source_type);

            // Add connection parameters
            ucdf.add_connection("url", &format!("{}://{}", protocol, host_port));
            if !path.is_empty() {
                ucdf.add_connection("path", path);
            }
            if !query.is_empty() {
                ucdf.add_connection("params", &query.replace('&', ","));
            }

            // Set access mode (assume read for API)
            ucdf.set_access_mode(AccessMode::Read);

            println!("{}", ucdf.to_string());
        }
        _ => {
            eprintln!("Error: Unsupported conversion from '{}' to '{}'", from, to);
            process::exit(1);
        }
    }
}

fn generate_command(source_type: &str) {
    let id = "";
    match source_type {
        "csv" => {
            println!("t=file.csv;c.path=/data/users.csv;c.encoding=utf-8;s.fields=id:int,name:str,email:str,created_at:date;a=r;m.desc=User data file");
        }
        "db" | "postgresql" => {
            println!("t=db.postgresql;c.host=localhost;c.port=5432;c.db=myapp;c.user=postgres;c.password=secret;s.fields=id:int,name:str,email:str;a=rw;m.desc=PostgreSQL database");
        }
        "api" | "rest" => {
            println!("t=api.rest;c.url=https://api.example.com;c.auth.type=bearer;c.auth.token=xyz123;s.endpoints=/users:GET,/users:POST,/users/{id}:GET,/users/{id}:PUT,/users/{id}:DELETE;a=rw;m.desc=REST API for user management");
        }
        "kafka" | "stream" => {
            println!("t=stream.kafka;c.brokers=broker1:9092,broker2:9092;c.topic=events;c.group_id=consumer_group_1;s.format=json;s.fields=event_id:str,timestamp:datetime,payload:json;a=r;m.desc=Kafka event stream");
        }
        "mongodb" => {
            println!("t=db.mongodb;c.uri=mongodb://localhost:27017;c.db=myapp;s.fields=_id:str,name:str,data:json;a=rw;m.desc=MongoDB database");
        }
        _ => {
            eprintln!("Error: Unknown source type '{}'", source_type);
            eprintln!("Available types: csv, db, postgresql, api, rest, kafka, stream, mongodb");
            process::exit(1);
        }
    }
}
