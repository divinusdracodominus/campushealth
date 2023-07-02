fn main() {
    //let dbstr;
    pgcodegen::reset_type_map().unwrap();
    let type_map = pgcodegen::laod_type_map("src/type_map.ron").unwrap();
    let mut builder = pgcodegen::Builder::new();

    builder
        .type_map(type_map)
        .derives(vec!["Debug", "Clone", "Serialize", "Deserialize"]);

    #[cfg(feature = "postgres")]
    builder
        .db_pull("postgresql://cardinal:Qksg0FV2EMDM@192.168.122.1:5432/myhealth")
        .unwrap();
    let generated = builder.generate().unwrap();
    //builder.db_push("postgresql://cardinal:Qksg0FV2EMDM@192.168.122.1:5432/temptest").unwrap();
    println!("{}", generated);
}
