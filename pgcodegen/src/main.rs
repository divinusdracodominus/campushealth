#[tokio::main]
async fn main() {
    //let dbstr;
    pgcodegen::reset_type_map().unwrap();
    let type_map = pgcodegen::laod_type_map("src/type_map.ron").unwrap();
    let mut builder = pgcodegen::Builder::new();
    let (mut client, conn) = tokio_postgres::connect(
        "postgresql://cardinal:Qksg0FV2EMDM@192.168.122.1:5432/myhealth",
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        if let Err(error) = conn.await {
            eprintln!("Connection error: {}", error);
        }
    });

    builder
        .type_map(type_map)
        .derives(vec!["Debug", "Clone", "Serialize", "Deserialize"]);

    #[cfg(feature = "postgres")]
    builder.db_builder().pg_db_pull(&mut client).await.unwrap();
    builder.generate().unwrap();
    let mut rocket = builder.into_rocket_builder();
    let rocket_fns = rocket.generate().unwrap().build();
    //builder.db_push("postgresql://cardinal:Qksg0FV2EMDM@192.168.122.1:5432/temptest").unwrap();
    println!("{}", rocket_fns);
}
