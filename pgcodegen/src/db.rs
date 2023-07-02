use crate::Table;
use codegen::*;
use std::collections::{HashMap, BTreeMap};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::{Column, Error};

pub struct DBBuilder<'a> {
    tables: &'a mut Vec<Table>,
    scope: &'a mut Scope,
    type_map: &'a HashMap<String, String>,
}

impl<'a> DBBuilder<'a> {
    pub(crate) fn create_from_parts(tables: &'a mut Vec<Table>, scope: &'a mut Scope, type_map: &'a HashMap<String, String>) -> Self {
        Self {
            tables,
            scope,
            type_map,
        }
    }
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

    pub fn generate(&mut self) -> Result<&mut Self, crate::Error> {
        self.generate_dbrow_trait();
        for table in self.tables.iter() {
            self.scope.push_impl(table.generate_dbrow_impl().unwrap());
        }
        //self.generate_dbrow_impl();
        Ok(self)
    }

    /// load data from currently a sync postgres client, support for future database engines will be added later
    #[cfg(feature = "postgres")]
    pub fn pg_db_pull(&mut self, url: &str) -> Result<&mut Self, Error> {
        use postgres::{Client, NoTls};
        let mut client = Client::connect(url, NoTls)?;

        let tables = client.simple_query(
            "select table_name from information_schema.tables where table_schema = 'public'",
        )?;
        let statement = client.prepare("select column_name,data_type,is_nullable from information_schema.columns where table_name = $1")?;
        let pkey_statement = client.prepare(include_str!("sql/get_primary_key.sql"))?;
        for table in tables.into_iter() {
            let table = match crate::postgresimpl::get_column(crate::postgresimpl::from_query_message(table), 0)? {
                Some(v) => v,
                None => continue,
            };
            let rows = client.query(&statement, &[&table])?;
            let mut columns = Vec::with_capacity(rows.len());
            for row in rows.into_iter() {
                let name = row.try_get(0)?;
                let data_type = row.try_get(1)?;
                let null = match row.try_get(2)? {
                    "YES" => true,
                    "NO" => false,
                    _ => false,
                };
                columns.push(Column::new(name, data_type, null, false));
            }
            let pkey = client.query(&pkey_statement, &[&table])?;
            let mut table = Table::new(table, columns);
            if let Some(pkey_column_ref) = pkey.into_iter().next() {
                let pkey_column = pkey_column_ref.try_get(0)?;
                table.primary_key(pkey_column)?;
            }

            self.tables.push(table);
        }
        Ok(self)
    }
}