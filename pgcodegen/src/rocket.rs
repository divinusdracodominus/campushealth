use crate::Table;
use codegen::Function;
use codegen::Scope;
use std::collections::HashMap;
pub struct RocketBuilder {
    tables: Vec<Table>,
    type_map: HashMap<String, String>,
    scope: Scope,
}

impl RocketBuilder {
    pub fn new(tables: Vec<Table>, type_map: HashMap<String, String>, scope: Scope) -> Self {
        Self { tables, type_map, scope }
    }

    pub fn generate(&mut self) -> Result<&mut Self, crate::Error> {
        for table in self.tables.iter() {
            if let Some(pkey_col) = &table.pkey_column() {
                
                let mut get_fn = Function::new(&format!("{}_rocket_get", table.name));
                get_fn.arg(&pkey_col.name, pkey_col.create_type(&self.type_map)?)
                    .attr(&format!("rocket::get(\"/{}/<{}>\", format = \"json\")", table.name, pkey_col.name))
                    .ret("rocket::response::content::RawJson<String>")
                    .line("let mut client = postgres::Client::connect(\"postgresql://cardinal:Qksg0FV2EMDM@192.168.122.1:5432/myhealth\", postgres::NoTls).unwrap();")
                    .line(format!("rocket::response::content::RawJson(serde_json::to_string(&{}::select(&mut client, &{}).unwrap().unwrap()).unwrap())", table.name, pkey_col.name));
                
                //let mut _post_fn = Function::new(&format!("{}_rocket_post", table.name));
                //let mut _put_fn = Function::new(&format!("{}_rocket_put", table.name));
                //let mut _delete_fn = Function::new(&format!("{}_rocket_delete", table.name));


                self.scope.push_fn(get_fn);
            }
        }
        Ok(self)
    }
    pub fn build(&self) -> String {
        self.scope.to_string()
    }
}
