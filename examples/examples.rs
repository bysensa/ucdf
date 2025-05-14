use ucdf::{
    parse, to_string, AccessMode, ConnectionParams, Endpoint, Field, Metadata, SourceType, UCDF,
};

fn main() {
    // Пример 1: Парсинг UCDF-строки
    println!("=== Пример 1: Парсинг UCDF-строки ===");
    let ucdf_str =
        "t=file.csv;c.path=/data/users.csv;s.fields=id:int,name:str,email:str;a=r;m.desc=User data";
    let ucdf = parse(ucdf_str).unwrap();

    println!("Тип источника: {}", ucdf.source_type);
    println!("Путь к файлу: {}", ucdf.connection.get("path").unwrap());
    println!("Режим доступа: {:?}", ucdf.access_mode.unwrap());
    println!("Описание: {}", ucdf.metadata.get("desc").unwrap());

    if let Some(fields) = ucdf.structure.get("fields") {
        println!("Структура:");
        println!("{:?}", fields);
    }

    println!("\nUCDF-строка: {}", ucdf.to_string());

    // Пример 2: Создание UCDF-структуры с помощью Builder
    println!("\n=== Пример 2: Создание с помощью Builder ===");
    let source_type = SourceType::builder()
        .category("db".to_string())
        .subtype("postgresql".to_string())
        .build();

    let ucdf = UCDF::builder()
        .source_type(source_type)
        .build()
        .with_connection("host", "localhost")
        .with_connection("port", "5432")
        .with_connection("user", "postgres")
        .with_connection("password", "secret")
        .with_connection("db", "mydb")
        .with_fields(vec![
            Field::builder()
                .name("id".to_string())
                .dtype("int".to_string())
                .build(),
            Field::builder()
                .name("name".to_string())
                .dtype("str".to_string())
                .build(),
            Field::builder()
                .name("created_at".to_string())
                .dtype("datetime".to_string())
                .build(),
        ])
        .with_access_mode(AccessMode::ReadWrite)
        .with_metadata("desc", "База данных пользователей")
        .with_metadata("owner", "admin");

    println!("UCDF-строка: {}", ucdf.to_string());

    // Пример 3: Создание UCDF для REST API
    println!("\n=== Пример 3: Создание UCDF для REST API ===");
    let source_type = SourceType::builder()
        .category("api".to_string())
        .subtype("rest".to_string())
        .build();

    let ucdf = UCDF::builder()
        .source_type(source_type)
        .build()
        .with_connection("url", "https://api.example.com/v1")
        .with_connection("auth.type", "bearer")
        .with_connection("auth.token", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")
        .with_endpoints(vec![
            Endpoint::builder()
                .path("/users".to_string())
                .method("GET".to_string())
                .build(),
            Endpoint::builder()
                .path("/users/{id}".to_string())
                .method("GET".to_string())
                .build(),
            Endpoint::builder()
                .path("/users".to_string())
                .method("POST".to_string())
                .build(),
            Endpoint::builder()
                .path("/users/{id}".to_string())
                .method("PUT".to_string())
                .build(),
        ])
        .with_access_mode(AccessMode::ReadWrite)
        .with_metadata("desc", "REST API для управления пользователями")
        .with_metadata("version", "1.0");

    println!("UCDF-строка: {}", ucdf.to_string());

    // Пример 4: Создание UCDF для Kafka
    println!("\n=== Пример 4: Создание UCDF для Kafka ===");
    let source_type = SourceType::builder()
        .category("stream".to_string())
        .subtype("kafka".to_string())
        .build();

    let ucdf = UCDF::builder()
        .source_type(source_type)
        .build()
        .with_connection("brokers", "kafka1:9092,kafka2:9092,kafka3:9092")
        .with_connection("topic", "users-events")
        .with_connection("group_id", "user-service")
        .with_format("json")
        .with_fields(vec![
            Field::builder()
                .name("event_id".to_string())
                .dtype("str".to_string())
                .build(),
            Field::builder()
                .name("event_type".to_string())
                .dtype("str".to_string())
                .build(),
            Field::builder()
                .name("user_id".to_string())
                .dtype("int".to_string())
                .build(),
            Field::builder()
                .name("data".to_string())
                .dtype("json".to_string())
                .build(),
            Field::builder()
                .name("timestamp".to_string())
                .dtype("datetime".to_string())
                .build(),
        ])
        .with_access_mode(AccessMode::Read)
        .with_metadata("desc", "Поток событий пользовательской активности")
        .with_metadata("retention", "7d");

    println!("UCDF-строка: {}", ucdf.to_string());

    // Пример 5: Модификация существующего UCDF
    println!("\n=== Пример 5: Модификация существующего UCDF ===");
    let mut ucdf = parse("t=file.csv;c.path=/data/users.csv").unwrap();

    // Добавляем новые параметры
    ucdf.add_connection("encoding", "utf-8");
    ucdf.add_fields(vec![
        Field::builder()
            .name("id".to_string())
            .dtype("int".to_string())
            .build(),
        Field::builder()
            .name("name".to_string())
            .dtype("str".to_string())
            .build(),
    ]);
    ucdf.set_access_mode(AccessMode::ReadWrite);
    ucdf.add_metadata("owner", "data-team");

    println!("Модифицированный UCDF: {}", ucdf.to_string());
}
