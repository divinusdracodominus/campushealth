/*!
    # Purpose

    This crate uses codegen to generate rust structures
    based off of a database schema, I found diesel a little convoluted, and overblown for the purpose I needed it for
    so instead resulted to using my own crate.

    ## Examples

    This example is one that can easily be embedded into a build.rs
    file to automate the process of updating schemas or APIs based on a postgres
    database schema.
    ```ignore
    use pgcodegen::*;

    use std::io::{Write};
    use std::path::PathBuf;

    fn main() {

        let dbstr = String::from("postgresql://username:password@host:port/database");
        let srcdir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let type_map = basic_typemap();
        let mut builder = Builder::new();
        builder.type_map(type_map)
            .db_pull(&dbstr)
            .unwrap()
            .derives(vec!["Debug", "Clone", "Serialize", "Deserialize"]);
        let generated = builder.generate().unwrap();
        let mut schema = std::fs::File::create(PathBuf::new().join(srcdir).join("src").join("schema.rs")).unwrap();
        schema.write_all(generated.as_bytes()).unwrap();
    }
    ```
**/

#[cfg(feature = "db")]
pub mod db;
use db::DBBuilder;

#[cfg(feature = "rocket")]
pub mod rocket;

#[cfg(feature = "postgres")]
pub mod postgresimpl;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate err_derive;

use codegen::{Field, Function, Impl, Scope, Struct, Type};
use std::io::{Read, Write};

use std::collections::HashMap;

use crate::rocket::RocketBuilder;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type Uuid = String;

pub fn new_v4() -> Uuid {
    uuid::Uuid::new_v4().to_string()
}

pub fn basic_typemap() -> HashMap<String, String> {
    let mut type_map = HashMap::new();
    type_map.insert(String::from("uuid"), String::from("uuid::Uuid"));
    type_map.insert(String::from("character varying"), String::from("String"));
    type_map.insert(String::from("boolean"), String::from("bool"));
    type_map.insert(String::from("bigint"), String::from("i64"));
    type_map.insert(String::from("integer"), String::from("i32"));
    type_map.insert(String::from("text"), String::from("String"));
    type_map.insert(String::from("date"), String::from("chrono::NaiveDate"));
    type_map.insert(String::from("real"), String::from("f32"));
    type_map.insert(String::from("smallint"), String::from("i16"));
    type_map.insert(String::from("smallserial"), String::from("u16"));
    type_map.insert(String::from("serial"), String::from("u32"));
    type_map.insert(String::from("bigserial"), String::from("u64"));
    type_map.insert(String::from("ARRAY"), String::from("Vec<String>"));
    type_map
}

/// this only makes sense for building source code
pub fn reset_type_map() -> Result<(), Error> {
    let type_map = basic_typemap();
    let data = ron::to_string(&type_map).unwrap();
    let mut file = std::fs::File::create("src/type_map.ron").unwrap();
    file.write_all(data.as_bytes()).unwrap();
    Ok(())
}

pub fn laod_type_map<P: AsRef<std::path::Path>>(path: P) -> Result<HashMap<String, String>, Error> {
    let mut content = String::new();
    let mut file = std::fs::File::open(path).unwrap();
    file.read_to_string(&mut content).unwrap();
    Ok(ron::from_str(&content).unwrap())
}

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "postgres")]
    #[source]
    #[error(display = "{}", _0)]
    Postgres(tokio_postgres::Error),
    #[source]
    #[error(display = "{}", _0)]
    IO(std::io::Error),
    #[error(display = "missing type: {}", _0)]
    MissingType(String),
    #[source]
    #[error(display = "{}", _0)]
    SerializeError(ron::Error),
    #[error(display = "column {} not found", _0)]
    MissingColumn(String),
    #[error(display = "not really an error, command completion: {}", _0)]
    CommandComplete(u64),
    #[error(display = "missing primary key for table: {}", _0)]
    MissingPKey(String),
}

impl From<tokio_postgres::Error> for Error {
    fn from(pgerr: tokio_postgres::Error) -> Error {
        Error::Postgres(pgerr)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Column {
    name: String,
    data_type: String,
    null: bool,
    primary_key: bool,
}

impl Column {
    pub fn new(name: String, data_type: String, null: bool, primary_key: bool) -> Self {
        Self {
            name,
            data_type,
            null,
            primary_key,
        }
    }

    pub fn create_type(&self, type_map: &HashMap<String, String>) -> Result<Type, Error> {
        Ok(Type::new(self.create_type_internal(type_map)?))
    }

    pub fn create_many_pkeys(&self, type_map: &HashMap<String, String>) -> Result<Type, Error> {
        let mut output = Type::new("Vec");
        output.generic(self.create_type_internal(type_map)?);
        Ok(output)
    }

    fn create_type_internal(&self, type_map: &HashMap<String, String>) -> Result<String, Error> {
        match type_map.get(&self.data_type) {
            Some(col_type) => {
                if self.null {
                    Ok(format!("Option<{}>", col_type))
                } else {
                    Ok(col_type.to_string())
                }
            }
            None => Err(Error::MissingType(self.data_type.clone())),
        }
    }

    pub fn create_where_type(&self, type_map: &HashMap<String, String>) -> Result<Type, Error> {
        Ok(Type::new(format!(
            "Option<{}>",
            self.create_type_internal(type_map)?
        )))
    }

    fn get_create_str(&self) -> String {
        if self.primary_key {
            format!("\"{}\" {} primary key", self.name, self.data_type)
        } else if self.null {
            format!("\"{}\" {} not null", self.name, self.data_type)
        } else {
            format!("\"{}\" {}", self.name, self.data_type)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    primary_key: Option<String>,
}

impl Table {
    fn where_name(&self) -> String {
        format!("Where{}", self.name)
    }

    pub fn pkey_column(&self) -> Option<&Column> {
        if let Some(pkey) = &self.primary_key {
            self.columns.iter().find(|c| &c.name == pkey)
        } else {
            None
        }
    }
    pub fn new<S: ToString>(name: S, columns: Vec<Column>) -> Self {
        Self {
            name: name.to_string(),
            columns,
            primary_key: None,
        }
    }

    // currently doesn't work make_bytes needs to be added to a trait, then implemented
    #[cfg(feature = "db")]
    fn where_list_fn(&self) -> Function {
        let mut outfun = Function::new("build_clause");
        outfun.arg_ref_self()
            .ret("(Vec<&'static str>, Vec<Vec<u8>>)")
            .line("let mut values: Vec<Vec<u8>> = Vec::new();")
            .line("let mut fields: Vec<&'static str> = Vec::new();");
        for column in self.columns.iter() {
            outfun.line(format!("if let Some(value) = self.{} {{", column.name))
                .line("values.push(value.make_bytes().to_vec());")
                .line(format!("fields.push(\"{}\");", column.name))
                .line("}");
        }
        outfun.line("(fields, values)");
        outfun
    }

    /// # Returns
    /// returns a MissingColumn error when the column specified wasn't found in this table
    pub fn primary_key(&mut self, key: String) -> Result<&mut Self, Error> {
        let mut found: usize = 0;
        for column in self.columns.iter_mut().filter(|c| c.name == key) {
            column.primary_key = true;
            found += 1;
        }
        if found == 0 {
            return Err(Error::MissingColumn(key));
        }
        self.primary_key = Some(key);
        Ok(self)
    }

    #[cfg(feature = "db")]
    fn generate_delete_query(&self, pkey: &str) -> String {
        format!("\"delete from {} where {} = $1\"", &self.name, pkey)
    }

    #[cfg(feature = "db")]
    fn generate_insert_query(&self) -> String {
        let fields = self
            .columns
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<&str>>();
        let args = (1..fields.len() + 1)
            .map(|f| format!("${}", f))
            .collect::<Vec<String>>();
        format!(
            "\"insert into {} ({}) values ({})\"",
            self.name,
            fields.join(", "),
            args.join(", ")
        )
    }

    #[cfg(feature = "db")]
    fn full_self_fields(&self) -> String {
        self.columns
            .iter()
            .map(|v| format!("&self.{}", v.name))
            .collect::<Vec<String>>()
            .join(", ")
    }

    #[cfg(feature = "db")]
    fn full_self_fields_with_pkey(&self, pkey: &str) -> String {
        let mut fields = self
            .columns
            .iter()
            .map(|v| format!("&self.{}", v.name))
            .collect::<Vec<String>>();
        fields.push(format!("&self.{}", pkey));
        fields.join(", ")
    }

    #[cfg(feature = "db")]
    fn generate_update_query(&self, pkey: &str) -> String {
        let sets = self
            .columns
            .iter()
            .enumerate()
            .map(|(idx, v)| format!("{} = ${}", v.name, idx + 1))
            .collect::<Vec<String>>();
        let last_arg = sets.len() + 1;
        format!(
            "\"update {} set {} where {} = ${}\"",
            self.name,
            sets.join(", "),
            pkey,
            last_arg
        )
    }

    #[cfg(feature = "postgres")]
    fn generate_dbrow_impl(&self) -> Result<Impl, Error> {
        if let Some(pkey) = &self.primary_key {
            let mut row_impl = Impl::new(&self.name);
            row_impl.r#macro("#[async_trait::async_trait]");

            let mut fn_map =
                DBBuilder::generate_db_row_fns(Type::new("&mut tokio_postgres::Client"), "tokio_postgres::types::ToSql");
            {
                let delete_fn = fn_map.get_mut("delete").unwrap();
                delete_fn.line(&format!(
                    "client.execute({}, &[&self.{}]).await",
                    self.generate_delete_query(pkey),
                    pkey
                ));
            }
            {
                let insert_fn = fn_map.get_mut("insert").unwrap();
                insert_fn.line(&format!(
                    "client.execute({}, &[{}]).await",
                    self.generate_insert_query(),
                    self.full_self_fields()
                ));
            }
            {
                let update_fn = fn_map.get_mut("update").unwrap();
                update_fn.line(&format!(
                    "client.execute({}, &[{}]).await",
                    self.generate_update_query(pkey),
                    self.full_self_fields_with_pkey(pkey)
                ));
            }
            {
                let select_fn = fn_map.get_mut("select").unwrap();
                select_fn.line(&format!("let selected = client.query(\"select {} from {} where {} = $1\", &[pkey]).await?;", self.columns.iter().map(|c| c.name.as_str()).collect::<Vec<&str>>().join(", "), self.name, pkey))
                    .line("let fields = match selected.get(0) {")
                    .line("Some(fields) => fields,")
                    .line("None => return Ok(None)")
                    .line("};")
                    .line("Ok(Some(Self { ");
                for (idx, column) in self.columns.iter().enumerate() {
                    select_fn.line(&format!("{}: fields.try_get({})?,", column.name, idx));
                }
                select_fn.line("}))");
            }
            /*{
                let select_all = fn_map.get_mut("select_all").unwrap();
                select_all.line(format!("client.simple_query(\"select * from {}\").await?;", self.name))
                    .line("unimplemented!()");
            }*/

            let mut dbrow_type = Type::from("DBRow");
            dbrow_type.generic("tokio_postgres::Client"); //.generic(format!("where_types::{}", self.where_name()));
            row_impl
                .impl_trait(dbrow_type)
                .associate_type("Err", Type::new("tokio_postgres::Error"));

            for (_k, func) in fn_map.into_iter() {
                if func.body.is_some() {
                    row_impl.push_fn(func);
                }
            }
            Ok(row_impl)
        } else {
            Err(Error::MissingPKey(self.name.clone()))
        }
    }

    fn generate_row_impl(&self, type_map: &HashMap<String, String>) -> Result<Impl, Error> {
        let mut where_impl = Impl::new(Type::new(&self.where_name()));

        /*let mut gen_clause_fn = Function::new("gen_clause");
        gen_clause_fn.arg_ref_self()
            .ret("Option<String>")
            .line("let mut fields: Vec<String> = String::new();");*/
        // remember to call to string

        for column in self.columns.iter() {
            /*gen_clause_fn.line(format!("if let Some(val) = self.{} {"))
            .line(format!("fields.push(\"{}\");", column.name))
            .line("}");*/
            let mut func = Function::new(&column.name);

            func.arg_mut_self()
                .arg(
                    &column.name,
                    Type::new(&column.create_type_internal(type_map)?),
                )
                .ret(Type::new("&mut Self"))
                .line(format!("self.{} = Some({});", column.name, column.name))
                .line("self")
                .vis("pub");
            where_impl.push_fn(func);
        }
        let mut where_kind_fn = Function::new("operator");
        where_kind_fn
            .arg("operator", Type::new("WhereType"))
            .arg_mut_self()
            .ret(Type::new("&mut Self"))
            .line("self.where_kind = operator;".to_string())
            .line("self")
            .vis("pub");

        where_impl.push_fn(where_kind_fn);
        //where_impl.push_fn(self.where_list_fn());
        Ok(where_impl)
    }

    /// impls From<Table> for WhereTable
    pub fn impl_from_struct(&self) -> Impl {
        let output = format!("super::{}", &self.name);
        let mut from_impl = Impl::new(&self.where_name());
        let mut from_trait = Type::new("From");
        from_trait.generic(&output);
        let mut from_fn = Function::new("from");
        from_fn
            .arg("other", Type::new(&output))
            .ret(Type::new(&self.where_name()));
        from_fn.line("Self {");
        for column in self.columns.iter() {
            from_fn.line(&format!("{}: Some(other.{}),", column.name, column.name));
        }
        from_fn.line("where_kind: WhereType::And");
        from_fn.line("}");
        from_impl.impl_trait(from_trait);
        from_impl.push_fn(from_fn);
        from_impl
    }

    pub fn generate_struct(
        &self,
        type_map: &HashMap<String, String>,
        derives: &[String],
    ) -> Result<Struct, Error> {
        let mut newstruct = Struct::new(&self.name);
        newstruct.vis("pub");
        for derive in derives.iter() {
            newstruct.derive(derive);
        }

        for column in self.columns.iter() {
            let mut field = Field::new(&column.name, column.create_type(type_map)?);
            field.vis("pub");
            newstruct.push_field(field);
        }
        Ok(newstruct)
    }

    #[cfg(feature = "db")]
    pub fn generate_where_struct(
        &self,
        type_map: &HashMap<String, String>,
        derives: &[String],
    ) -> Result<Struct, Error> {
        let mut wherestruct = Struct::new(&self.where_name());
        wherestruct.vis("pub");
        wherestruct.doc(&format!("used as where clause for type: {} \n Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.", self.name));

        for derive in derives.iter() {
            wherestruct.derive(derive);
        }

        for column in self.columns.iter() {
            let mut wherefield = Field::new(&column.name, column.create_where_type(type_map)?);
            wherefield.vis("pub");
            wherestruct.push_field(wherefield);
        }
        wherestruct.new_field("where_kind", Type::new("WhereType"));
        
        Ok(wherestruct)
    }
}

impl std::default::Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

/// The primary builder struct that leverages codegen
/// its primary purpose is to load schemas, and generate resulting rust code
#[derive(Debug, Clone)]
pub struct Builder {
    scope: Scope,
    tables: Vec<Table>,
    type_map: HashMap<String, String>,
    derives: Vec<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            type_map: HashMap::new(),
            derives: Vec::new(),
            scope: Scope::new(),
        }
    }

    ///
    /// # Arguments
    ///
    /// type_map:
    /// a HashMap that represents the mapping from postgres type (or databse type when other dbs are added)
    /// to rust types, currently however no type resolution is employed, so the types may be arbitrary strings
    /// for example uuid::Uuid would require uuid crate, which may not be a dependency of the project
    /// this code is used in.
    pub fn type_map(&mut self, type_map: HashMap<String, String>) -> &mut Self {
        self.type_map = type_map;
        self
    }

    /// this specifies the derives that should be applied to all structs
    /// future implementations may include per struct derives, but currently that isn't supported
    pub fn derives<T: ToString>(&mut self, derives: Vec<T>) -> &mut Self {
        for derive in derives.into_iter() {
            self.derives.push(derive.to_string());
        }
        self
    }

    pub fn db_builder(&mut self) -> DBBuilder {
        DBBuilder::create_from_parts(
            &mut self.tables,
            &mut self.scope,
            &self.type_map,
            &self.derives,
        )
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

    /// the primary method this struct employs which converts
    /// the tables, derives, scope, and type map into corresponding rust code
    pub fn generate(&mut self) -> Result<&mut Self, Error> {
        for table in self.tables.iter() {
            self.scope
                .push_struct(table.generate_struct(&self.type_map, &self.derives)?);
        }
        Ok(self)
    }

    pub fn build(&self) -> String {
        self.scope.to_string()
    }

    pub fn into_rocket_builder(self) -> RocketBuilder {
        RocketBuilder::new(self.tables, self.type_map, self.scope)
    }
}
