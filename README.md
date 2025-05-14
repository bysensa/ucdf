# UCDF - Unified Compact Data Format for Rust

[![Latest Version](https://img.shields.io/crates/v/ucdf.svg)](https://crates.io/crates/ucdf)
[![Documentation](https://docs.rs/ucdf/badge.svg)](https://docs.rs/ucdf)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A Rust implementation of the Unified Compact Data Format (UCDF) - a universal compact format for describing data sources in a single line.

## Overview

UCDF is designed for use in configurations, logs, CLI tools, and other scenarios where compact and unified data source descriptions are important. The format combines ease of use with expressive power to describe various data sources in a standardized way.

## Features

- **Compact representation** of data sources in a single line
- Support for various **source types** (files, databases, APIs, message streams)
- Structured format with **sections** for different aspects of a data source
- Extensible design that can be adapted to multiple use cases
- **Type-safe** Rust implementation with comprehensive error handling
- **Serialization** and **deserialization** support
- Builder pattern API for easy construction

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ucdf = "0.1.0"
```

## Quick Start

```rust
use ucdf::{parse, UCDF, SourceType, AccessMode, Field};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a UCDF string
    let ucdf_str = "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str;a=r";
    let ucdf = ucdf::parse(ucdf_str)?;

    println!("Source type: {}", ucdf.source_type);
    println!("Path: {}", ucdf.connection.get("path").unwrap_or(&"".to_string()));

    // Create a UCDF string programmatically
    let source_type = SourceType::new("db".to_string(), Some("postgresql".to_string()));
    let ucdf = UCDF::with_source_type(source_type)
        .with_connection("host", "localhost")
        .with_connection("port", "5432")
        .with_connection("db", "myapp")
        .with_fields(vec![
            Field::new("id".to_string(), "int".to_string(), None),
            Field::new("name".to_string(), "str".to_string(), None),
        ])
        .with_access_mode(AccessMode::ReadWrite);

    // Convert back to string
    let ucdf_str = ucdf.to_string();
    println!("UCDF: {}", ucdf_str);

    Ok(())
}
```

## UCDF Format Specification

UCDF consists of sections separated by semicolons (`;`), where each section contains key-value pairs.

### Format Structure

```
t=<type>;[c.<param>=<value>];[s.<structure>=<description>];[a=<access>];[m.<meta>=<value>]
```

### Section Types

- **Type (`t`)**: Defines the data source type (required)

  - Example: `t=file.csv`, `t=db.postgresql`, `t=api.rest`

- **Connection (`c`)**: Connection parameters

  - Example: `c.path=/data/users.csv`, `c.host=localhost`

- **Structure (`s`)**: Data structure or schema

  - Example: `s.fields=id:int,name:str`, `s.endpoints=/users:GET`

- **Access (`a`)**: Access mode

  - Values: `r` (read), `w` (write), `rw` (read-write)

- **Metadata (`m`)**: Additional information
  - Example: `m.desc=User data`, `m.owner=admin`

### Examples

#### CSV File

```
t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str,email:str;a=r;m.desc=User data
```

#### PostgreSQL Database

```
t=db.postgresql;c.host=db.prod;c.user=readonly;c.db=sales;s.fields=id:int,amount:float,date:date;a=r
```

#### REST API

```
t=api.rest;c.url=https://api.example.com;c.auth.type=bearer;c.auth.token=xyz;s.endpoints=/users:GET,/orders:POST;a=rw
```

#### Kafka Stream

```
t=stream.kafka;c.brokers=broker1:9092,broker2:9092;c.topic=events;s.format=json;s.fields=event_id:str,payload:json;a=r
```

## Advanced Usage

See the examples directory for more advanced usage scenarios:

- `basic_usage.rs`: Basic parsing and creation of UCDF strings
- `advanced_usage.rs`: Advanced features and custom configurations
- `format_conversion.rs`: Converting between UCDF and other formats
- `build_ucdf.rs`: Building complex UCDF structures from scratch
- `ucdf_cli.rs`: Command-line interface for UCDF manipulation

## CLI Tool

The library includes a CLI tool example for working with UCDF strings:

```
# Parsing a UCDF string
cargo run --example ucdf_cli parse "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str;a=r"

# Converting from JDBC URL to UCDF
cargo run --example ucdf_cli convert jdbc ucdf "jdbc:postgresql://localhost:5432/mydb?user=postgres&password=secret"

# Converting from UCDF to URL
cargo run --example ucdf_cli convert ucdf url "t=api.rest;c.url=https://api.example.com;c.path=/users;c.params=limit=100"
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
