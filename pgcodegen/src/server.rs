use crate::Table;
use codegen::*;
use std::collections::{HashMap, BTreeMap};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct ServerBuilder<'a> {
    tables: &'a mut Vec<Table>,
    scope: &'a mut Scope,
    type_map: &'a HashMap<String, String>,
}

impl<'a> ServerBuilder<'a> {
    #[cfg(feature = "postgres")]
    pub fn create_pg_client<S: ToString>(&mut self, url: S) -> &mut Self {
        self.scope.raw(format!("
        lazy_static! {{
            static ref CLIENT: Arc<Mutex<postgres::Client>> = {{
                Arc::new(Mutex::new(postgres::Client::connect(
                    {},
                    postgres::NoTls,
                ).unwrap()))
            }};
        }}
        ", url.to_string()));
        self
    }

    fn generate_db_row_fns() -> BTreeMap<&'static str, Function> {
        let mut outmap = BTreeMap::new();
        let mut result = Type::new("Result");
        result
            .generic(Type::new("u64"))
            .generic(Type::new("Self::Err"));

        let mut select_result = Type::new("Result");
        let mut self_vec = Type::new("Option");
        self_vec.generic("Self");
        select_result.generic(self_vec).generic("Self::Err");

        let one = vec!["insert", "update", "delete"];
        for field in one.iter() {
            let mut func = Function::new(field.to_string());
            func.arg("client", Type::new("&mut T"))
                .arg_ref_self()
                .ret(&result);
            func.body = None;
            outmap.insert(*field, func);
        }
        
        let mut select_one = Function::new("select");
        select_one
            .arg("client", Type::new("&mut T"))
            .generic("P")
            .arg("pkey", Type::new("&P"))
            .ret(select_result)
            .bound("P", Type::new("postgres::types::ToSql"))
            .bound("P", Type::new("std::marker::Sync"))
            .doc("creates Self based on the primary key value passed into the function");
        select_one.body = None;
        outmap.insert("select", select_one);
        outmap
    }

    /// uses Hash trait to generate a hash of the tables, thereby producing a version number
    pub fn version_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.tables.hash(&mut s);
        s.finish()
    }

    pub fn version_hex(&self) -> String {
        hex::encode(self.version_hash().to_le_bytes())
    }

    pub fn version_base64(&self) -> String {
        use base64::engine::Engine;
        let custom = base64::alphabet::Alphabet::new(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._",
        )
        .unwrap();

        let engine =
            base64::engine::GeneralPurpose::new(&custom, base64::engine::general_purpose::PAD);
        engine.encode(&self.version_hash().to_le_bytes())
    }

    fn generate_dbrow_trait(&mut self) -> &mut Self {
        let mut dbrow_trait = Trait::new("DBRow");
        dbrow_trait
            .vis("pub")
            .generic("T")
            .generic("W")
            .bound("Self", "Sized")
            .associated_type("Err")
            .bound(Type::new("std::error::Error"));
        dbrow_trait.doc("the primary trait that is used for ORM CRUD operations, ## NOTE: Future plans include a default impl for insert_one, update_one, delete_one, select_one that use self.into::<WhereSelf>()");
        for (_key, func) in Self::generate_db_row_fns().into_iter() {
            dbrow_trait.push_fn(func);
        }

        let mut version_fn = Function::new("version_hash");
        version_fn.ret(Type::new("&'static str"))
            .line(&format!("\"{}\"", self.version_base64()))
            .vis("pub")
            .doc("base64 encoded hash of table names, column names, and column types using alphanumerics, ~, _");
        self.scope.push_fn(version_fn);
        self.scope.push_trait(dbrow_trait);
        self
    }
}