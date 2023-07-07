use crate::Table;
use crate::{Column, Error};
use codegen::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

pub struct DBBuilder<'a> {
    tables: &'a mut Vec<Table>,
    scope: &'a mut Scope,
    type_map: &'a HashMap<String, String>,
    derives: &'a [String],
}

impl<'a> DBBuilder<'a> {
    pub(crate) fn create_from_parts(
        tables: &'a mut Vec<Table>,
        scope: &'a mut Scope,
        type_map: &'a HashMap<String, String>,
        derives: &'a [String],
    ) -> Self {
        Self {
            tables,
            scope,
            type_map,
            derives,
        }
    }
    #[cfg(feature = "postgres")]
    pub fn create_pg_client<S: ToString>(&mut self, url: S) -> &mut Self {
        self.scope.raw(format!("
            async fn create_client() -> tokio_postgres::Client {{
                let (mut client, conn) = tokio_postgres::connect(\"{}\", tokio_postgres::NoTls).await.unwrap();

                tokio::spawn(async move {{
                    if let Err(error) = conn.await {{
                      eprintln!(\"Connection error: {{}}\", error);
                    }}
                  }});

                client
            }}
        ", url.to_string()));
        self
    }

    fn db_row_result_type() -> Type {
        let mut result = Type::new("Result");
        result
            .generic(Type::new("u64"))
            .generic(Type::new("Self::Err"));
        result
    }

    #[cfg(feature = "postgres")]
    fn impl_try_from_cols(&self) -> Impl {
        unimplemented!();
    }

    pub(crate) fn generate_db_row_fns(client_type: Type, bound: &str) -> BTreeMap<&'static str, Function> {
        let mut outmap = BTreeMap::new();
        let result = Self::db_row_result_type();

        let mut select_result = Type::new("Result");
        let mut self_vec = Type::new("Option");
        self_vec.generic("Self");
        select_result.generic(self_vec).generic("Self::Err");

        let one = vec!["insert", "update", "delete"];
        for field in one.iter() {
            let mut func = Function::new(field.to_string());
            func.arg("client", &client_type)
                .arg_ref_self()
                .ret(&result)
                .set_async(true);
            func.body = None;
            outmap.insert(*field, func);
        }

        let mut select_one = Function::new("select");
        select_one
            .arg("client", &client_type)
            .generic("P")
            .arg("pkey", Type::new("&P"))
            .ret(select_result)
            .set_async(true)
            .bound("P", Type::new(bound))
            .bound("P", Type::new("std::marker::Sync"))
            .doc("creates Self based on the primary key value passed into the function");
        select_one.body = None;
        outmap.insert("select", select_one);
        
        let mut select_all = Function::new("select_all");
            select_all.set_async(true)
            .arg("client", &client_type)
            .ret(Type::new("Result<Vec<Self>, Self::Err>"))
            .doc("select every entry in the table, probably shouldn't be used for large tables");
        select_all.body = None;
        //outmap.insert("select_all", select_all);
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
        engine.encode(self.version_hash().to_le_bytes())
    }
    #[cfg(feature = "postgres")]
    fn generate_dbrow_trait(&mut self) -> &mut Self {
        let mut dbrow_trait = Trait::new("DBRow");
        dbrow_trait
            .vis("pub")
            .attr("async_trait::async_trait")
            .generic("T")
            .bound("Self", "Sized")
            .bound("T", "std::marker::Send")
            .bound("Self", "std::marker::Sync")
            .associated_type("Err")
            .bound(Type::new("std::error::Error"));
        dbrow_trait.doc("the primary trait that is used for ORM CRUD operations, ## NOTE: Future plans include a default impl for insert_one, update_one, delete_one, select_one that use self.into::<WhereSelf>()");
        for (_key, func) in Self::generate_db_row_fns(Type::new("&mut T"), "tokio_postgres::types::ToSql").into_iter() {
            dbrow_trait.push_fn(func);
        }

        let mut insert_many = Function::new("insert_many");
        insert_many
            .set_async(true)
            .arg("client", Type::new("&mut T"))
            .arg("values", Type::new("&[Self]"))
            .line("for value in values.iter() {")
            .line("     let fut = value.insert(client);")
            .line("     fut.await?;")
            .line("}")
            .line("Ok(values.len() as u64)")
            .ret(Self::db_row_result_type());

        let mut update_many = Function::new("update_many");
        update_many
            .set_async(true)
            .arg("client", Type::new("&mut T"))
            .arg("values", Type::new("&[Self]"))
            .line("for value in values.iter() {")
            .line("     let fut = value.update(client);")
            .line("     fut.await?;")
            .line("}")
            .line("Ok(values.len() as u64)")
            .ret(Self::db_row_result_type());

        let mut delete_many = Function::new("delete_many");
        delete_many
            .set_async(true)
            .arg("client", Type::new("&mut T"))
            .arg("values", Type::new("&[Self]"))
            .line("for value in values.iter() {")
            .line("     let fut = value.delete(client);")
            .line("     fut.await?;")
            .line("}")
            .line("Ok(values.len() as u64)")
            .ret(Self::db_row_result_type());
        
        let mut select_many = Function::new("select_many");
        select_many
            .set_async(true)
            .arg("client", Type::new("&mut T"))
            .arg("pkeys", Type::new("&[P]"))
            .line("let mut outvec = Vec::with_capacity(pkeys.len());")
            .line("for value in pkeys.iter() {")
            .line("     let fut = Self::select(client, value);")
            .line("     outvec.push(fut.await?);")
            .line("}")
            .line("Ok(outvec)")
            .ret(Type::new("Result<Vec<Option<Self>>, Self::Err>"))
            .generic("P")
            .bound("P", Type::new("tokio_postgres::types::ToSql"))
            .bound("P", Type::new("std::marker::Sync"))
            .bound("P", Type::new("std::marker::Send"))
            .doc("creates Self based on the primary key value passed into the function");
    

        dbrow_trait.push_fn(insert_many);
        dbrow_trait.push_fn(update_many);
        dbrow_trait.push_fn(select_many);
        dbrow_trait.push_fn(delete_many);
        let mut version_fn = Function::new("version_hash");
        version_fn.ret(Type::new("&'static str"))
            .line(&format!("\"{}\"", self.version_base64()))
            .vis("pub")
            .doc("base64 encoded hash of table names, column names, and column types using alphanumerics, ~, _");
        self.scope.push_fn(version_fn);
        self.scope.push_trait(dbrow_trait);
        self
    }

    pub fn add_where_types(&mut self) -> Result<&mut Self, crate::Error> {
        self.scope.push_module(self.generate_where_module()?);
        Ok(self)
    }

    fn generate_sql_compat() -> Trait {
        let sqlcompat = Trait::new("SqlCompat");
        sqlcompat
    }

    pub fn generate<S: ToString>(&mut self, url: S) -> Result<&mut Self, crate::Error> {
        self.create_pg_client(url);
        self.generate_dbrow_trait();
        self.scope.push_trait(Self::generate_sql_compat());
        for table in self.tables.iter() {
            self.scope.push_impl(table.generate_dbrow_impl().unwrap());
        }
        //self.generate_dbrow_impl();
        Ok(self)
    }

    ///
    /// currently doesn't fully function because it can't handle the ARRAY type
    ///
    #[cfg(feature = "postgres")]
    pub async fn pg_db_push(
        &mut self,
        client: &mut tokio_postgres::Client,
    ) -> Result<(), tokio_postgres::error::Error> {
        for table in self.tables.iter() {
            let fields: String = table
                .columns
                .iter()
                .map(|c| c.get_create_str())
                .collect::<Vec<String>>()
                .join(", ")
                .to_string();
            let query = format!("create table {} ({})", table.name, fields);
            client.execute(&query, &[]).await?;
        }

        Ok(())
    }

    /// load data from currently a sync tokio_postgres client, support for future database engines will be added later
    #[cfg(feature = "postgres")]
    pub async fn pg_db_pull(&mut self, client: &mut tokio_postgres::Client) -> Result<(), Error> {
        let tables = client
            .simple_query(
                "select table_name from information_schema.tables where table_schema = 'public'",
            )
            .await?;
        let statement = client.prepare("select column_name,data_type,is_nullable from information_schema.columns where table_name = $1").await?;
        let pkey_statement = client
            .prepare(include_str!("sql/get_primary_key.sql"))
            .await?;
        for table in tables.into_iter() {
            let table = match crate::postgresimpl::get_column(
                crate::postgresimpl::from_query_message(table),
                0,
            )? {
                Some(v) => v,
                None => continue,
            };
            let rows = client.query(&statement, &[&table]).await?;
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
            let pkey = client.query(&pkey_statement, &[&table]).await?;
            let mut table = Table::new(table, columns);
            if let Some(pkey_column_ref) = pkey.into_iter().next() {
                let pkey_column = pkey_column_ref.try_get(0)?;
                table.primary_key(pkey_column)?;
            }

            self.tables.push(table);
        }
        Ok(())
    }

    /// creates a WhereType enum, and the implementation of Default for it
    fn create_where_enum(derives: &[String]) -> Enum {
        let mut where_enum = Enum::new("WhereType");
        where_enum.new_variant("Or");
        where_enum.new_variant("And").annotation("#[default]");
        where_enum.vis("pub");
        for derive in derives {
            where_enum.derive(derive);
        }

        let mut default_impl = Impl::new("WhereType");
        default_impl.impl_trait(Type::new("std::default::Default"));
        default_impl
            .new_fn("default")
            .ret("Self")
            .line("Self::None");

        where_enum
    }

    fn generate_where_module(&self) -> Result<Module, Error> {
        let mut where_module = Module::new("where_types");
        //where_module.doc("Provides the where clause types for this ORM");
        where_module.vis("pub");

        for table in self.tables.iter() {
            let wherestruct = table.generate_where_struct(self.type_map, self.derives)?;
            where_module.push_struct(wherestruct);
            where_module.push_impl(table.generate_row_impl(self.type_map)?);
            where_module.push_impl(table.impl_from_struct());
        }
        let where_enum = Self::create_where_enum(self.derives);
        where_module.push_enum(where_enum);

        Ok(where_module)
    }
}
