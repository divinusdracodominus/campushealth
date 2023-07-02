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

#[cfg(feature = "postgres")]
pub mod db;

#[cfg(feature = "rocket")]
pub mod rocket;
pub mod server;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate err_derive;

use codegen::{Field, Enum, Function, Impl, Module, Scope, Struct, Trait, Type};
#[cfg(feature = "postgres")]
use postgres::{Client, NoTls};
use std::io::{Read, Write};

use std::collections::{HashMap, BTreeMap};

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
    Postgres(postgres::Error),
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

impl From<postgres::Error> for Error {
    fn from(pgerr: postgres::Error) -> Error {
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
            self.columns.iter().filter(|c| &c.name == pkey).next()
        }else{
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

    fn generate_delete_query(&self, pkey: &str) -> String {
        format!("\"delete from {} where {} = $1\"", &self.name, pkey)
    }

    fn generate_insert_query(&self) -> String {
        let fields = self.columns.iter().map(|c| c.name.as_str()).collect::<Vec<&str>>();
        let args = (1..fields.len()+1).map(|f| format!("${}", f)).collect::<Vec<String>>();
        format!("\"insert into {} ({}) values ({})\"", self.name, fields.join(", "), args.join(", "))
    }

    fn full_self_fields(&self) -> String {
        self.columns.iter().map(|v| format!("&self.{}", v.name)).collect::<Vec<String>>().join(", ")
    }

    fn full_self_fields_with_pkey(&self, pkey: &str) -> String {
        let mut fields = self.columns.iter().map(|v| format!("&self.{}", v.name)).collect::<Vec<String>>();
        fields.push(format!("&self.{}", pkey));
        fields.join(", ")
    }

    fn generate_update_query(&self, pkey: &str) -> String {
        let sets = self.columns.iter().enumerate().map(|(idx, v)| format!("{} = ${}", v.name, idx+1)).collect::<Vec<String>>();
        let last_arg = sets.len()+1;
        format!("\"update {} set {} where {} = ${}\"", self.name, sets.join(", "), pkey, last_arg)
    }

    fn generate_dbrow_impl(&self) -> Result<Impl, Error> {
        if let Some(pkey) = &self.primary_key {
            let mut row_impl = Impl::new(&self.name);
            
            

            let mut fn_map = Builder::generate_db_row_fns("client");
            {
                let delete_fn = fn_map.get_mut("delete").unwrap();
                delete_fn.line(&format!("client.exec({}, &[&self.{}])", self.generate_delete_query(&pkey), pkey));
            }
            {
                let insert_fn = fn_map.get_mut("insert").unwrap();
                insert_fn.line(&format!("client.exec({}, &[{}])", self.generate_insert_query(), self.full_self_fields()));
            }
            {
                let update_fn = fn_map.get_mut("update").unwrap();
                update_fn.line(&format!("client.exec({}, &[{}])", self.generate_update_query(pkey), self.full_self_fields_with_pkey(pkey)));
            }
            {
                let select_fn = fn_map.get_mut("select").unwrap();
                select_fn.line(&format!("let selected = client.fetch(\"select {} from {} where {} = $1\", &[pkey])?;", self.columns.iter().map(|c| c.name.as_str()).collect::<Vec<&str>>().join(", "), self.name, pkey))
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

            let mut dbrow_type = Type::from("DBRow");
            dbrow_type.generic("T").generic(format!("where_types::{}", self.where_name()));
            row_impl.impl_trait(dbrow_type)
                .associate_type("Err", Type::new("<T as pgcodegen::db::SimpleClient>::Err"))
                .bound("T", "pgcodegen::db::SimpleClient<Row = postgres::Row, Err = postgres::Error>")
                .generic("T");
            
            for (_k, func) in fn_map.into_iter() {
                if func.body.is_some() {
                    row_impl.push_fn(func);
                }
            }
            Ok(row_impl)
        } else {
            return Err(Error::MissingPKey(self.name.clone()))
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
        where_kind_fn.arg("operator", Type::new("WhereType"))
            .arg_mut_self()
            .ret(Type::new("&mut Self"))
            .line(format!("self.where_kind = operator;"))
            .line("self")
            .vis("pub");

        where_impl.push_fn(where_kind_fn);
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

    /// load data from currently a sync postgres client, support for future database engines will be added later
    #[cfg(feature = "postgres")]
    pub fn db_pull(&mut self, url: &str) -> Result<&mut Self, Error> {
        let mut client = Client::connect(url, NoTls)?;

        let tables = client.simple_query(
            "select table_name from information_schema.tables where table_schema = 'public'",
        )?;
        let statement = client.prepare("select column_name,data_type,is_nullable from information_schema.columns where table_name = $1")?;
        let pkey_statement = client.prepare(include_str!("sql/get_primary_key.sql"))?;
        for table in tables.into_iter() {
            let table = match db::get_column(db::from_query_message(table), 0)? {
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

    ///
    /// currently doesn't fully function because it can't handle the ARRAY type
    ///
    #[cfg(feature = "postgres")]
    pub fn db_push(&mut self, url: &str) -> Result<(), postgres::error::Error> {
        let mut client = Client::connect(url, NoTls)?;

        for table in self.tables.iter() {
            let fields: String = table
                .columns
                .iter()
                .map(|c| c.get_create_str())
                .collect::<Vec<String>>()
                .join(", ")
                .to_string();
            let query = format!("create table {} ({})", table.name, fields);
            client.execute(&query, &vec![])?;
        }

        Ok(())
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
        default_impl.new_fn("default")
            .ret("Self")
            .line("Self::None");

        where_enum
    }

    /// the primary method this struct employs which converts
    /// the tables, derives, scope, and type map into corresponding rust code
    pub fn generate(&mut self) -> Result<&mut Self, Error> {
        let mut where_module = Module::new("where_types");
        //where_module.doc("Provides the where clause types for this ORM");
        where_module.vis("pub");

        for table in self.tables.iter() {
            let mut newstruct = Struct::new(&table.name);
            let mut wherestruct = Struct::new(&table.where_name());

            newstruct.vis("pub");
            wherestruct.vis("pub");
            wherestruct.doc(&format!("used as where clause for type: {} \n Each field that appears in the generated type is wrapped in an Option that determines wether or not it should appear in the where clause, nested options are treated as nullable arguments in the where clause.", table.name));

            for derive in self.derives.iter() {
                newstruct.derive(derive);
                wherestruct.derive(derive);
            }

            for column in table.columns.iter() {
                let mut field = Field::new(&column.name, column.create_type(&self.type_map)?);
                field.vis("pub");
                let mut wherefield =
                    Field::new(&column.name, column.create_where_type(&self.type_map)?);
                wherefield.vis("pub");

                newstruct.push_field(field);

                wherestruct.push_field(wherefield);
            }
            self.scope.push_struct(newstruct);
            self.scope.push_impl(table.generate_dbrow_impl().unwrap());
            wherestruct.new_field("where_kind", Type::new("WhereType"));
            where_module.push_struct(wherestruct);

            where_module.push_impl(table.generate_row_impl(&self.type_map)?);
            where_module.push_impl(table.impl_from_struct());
        }

        let where_enum = Self::create_where_enum(&self.derives);
        where_module.push_enum(where_enum);

        self.scope.push_module(where_module);
        //self.generate_dbrow_trait();
        Ok(self)
    }

    pub fn build(&self) -> String {
        self.scope.to_string()
    }

    fn generate_db_row_fns(client_name: &'static str) -> BTreeMap<&'static str, Function> {
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
            func.arg(client_name, Type::new("&mut T"))
                .arg_ref_self()
                .ret(&result);
            func.body = None;
            outmap.insert(*field, func);
        }
        
        let mut select_one = Function::new("select");
        select_one
            .arg(client_name, Type::new("&mut T"))
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

    /*fn generate_dbrow_trait(&mut self) -> &mut Self {
        let mut dbrow_trait = Trait::new(DBROWTRAIT);
        dbrow_trait
            .vis("pub")
            .generic("T")
            .generic("W")
            .bound("Self", "Sized")
            .associated_type("Err")
            .bound(Type::new("std::error::Error"));
        dbrow_trait.doc("the primary trait that is used for ORM CRUD operations, ## NOTE: Future plans include a default impl for insert_one, update_one, delete_one, select_one that use self.into::<WhereSelf>()");
        for (_key, func) in Self::generate_db_row_fns("client").into_iter() {
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
    }*/

    pub fn into_rocket_builder(self) -> RocketBuilder {
        RocketBuilder::new(self.tables, self.type_map, self.scope)
    }
}
