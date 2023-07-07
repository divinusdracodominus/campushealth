use crate::Table;
use codegen::Function;
use codegen::Scope;
use codegen::Type;
use std::collections::HashMap;
pub struct RocketBuilder {
    tables: Vec<Table>,
    type_map: HashMap<String, String>,
    scope: Scope,
}

impl RocketBuilder {
    pub fn new(tables: Vec<Table>, type_map: HashMap<String, String>, scope: Scope) -> Self {
        Self {
            tables,
            type_map,
            scope,
        }
    }

    fn create_launcher(&self) -> Function {
        let mut mounts = Function::new("mount_routes");
        mounts
            .arg("mut rocket", Type::new("rocket::Rocket<rocket::Build>"))
            .ret(Type::new("rocket::Rocket<rocket::Build>"))
            .line("rocket")
            .vis("pub");

        for table in self.tables.iter() {
            for route in Self::get_route_names(&table.name).into_iter() {
                mounts.line(format!("   .mount(\"/api/\", rocket::routes![{}])", route));
            }
        }

        //mounts.line("; rocket");

        mounts
    }

    fn get_route_names(name: &str) -> Vec<String> {
        vec![
            format!("{}_rocket_get", name),
            format!("{}_rocket_delete", name),
            format!("{}_rocket_post", name),
            format!("{}_rocket_put", name),
        ]
    }

    pub fn generate(&mut self) -> Result<&mut Self, crate::Error> {
        self.scope.push_fn(self.create_launcher());
        for table in self.tables.iter() {
            if let Some(pkey_col) = &table.pkey_column() {
                //let mut pkey_wrapper = Struct::new("pkey_")
                let alias_name = format!("{}_pkey_list", table.name);
                self.scope.new_type_alias(&alias_name, format!("Vec<{}>", pkey_col.create_type_internal(&self.type_map)?));
                let mut get_fn = Function::new(&format!("{}_rocket_get", table.name));
                get_fn.arg(&pkey_col.name, pkey_col.create_type(&self.type_map)?)
                    .set_async(true)
                    .attr(&format!("rocket::get(\"/{}/by-id/<{}>\", format = \"json\")", table.name, pkey_col.name))
                    .ret("rocket::response::content::RawJson<String>")
                    .line("let mut client = create_client().await;")
                    .line(format!("rocket::response::content::RawJson(serde_json::to_string(&{}::select(&mut client, &{}).await.unwrap().unwrap()).unwrap())", table.name, pkey_col.name));

                let mut post_fn = Function::new(&format!("{}_rocket_post", table.name));
                post_fn.arg(&table.name, Type::new(format!("rocket::serde::json::Json<{}>", table.name)))
                    .set_async(true)
                    .attr(&format!("rocket::post(\"/{}/instance/\", format = \"json\", data = \"<{}>\")", table.name, table.name))
                    .line("let mut client = create_client().await;")
                    .line(format!("{}.0.insert(&mut client).await.unwrap();", table.name));
                let mut put_fn = Function::new(&format!("{}_rocket_put", table.name));
                put_fn.arg(&table.name, Type::new(format!("rocket::serde::json::Json<{}>", table.name)))
                    .set_async(true)
                    .attr(&format!("rocket::put(\"/{}/instance/\", format = \"json\", data = \"<{}>\")", table.name, table.name))
                    .line("let mut client = create_client().await;")
                    .line(format!("{}.0.update(&mut client).await.unwrap();", table.name));

                let mut delete_fn = Function::new(&format!("{}_rocket_delete", table.name));
                delete_fn.arg(&pkey_col.name, pkey_col.create_type(&self.type_map)?)
                    .set_async(true)
                    .attr(&format!(
                        "rocket::delete(\"/{}/by-id/<{}>\")",
                        table.name, pkey_col.name
                    ))
                    .ret("rocket::response::content::RawText<&'static str>")
                    .line("let mut client = create_client().await;")
                    .line(format!("let mut todelete = {}::default();", table.name))
                    .line(format!("todelete.{} = {};", pkey_col.name, pkey_col.name))
                    .line("todelete.delete(&mut client).await.unwrap();")
                    .line("rocket::response::content::RawText(\"it worked\")");
                
                    let arg_name = format!("{}_list", pkey_col.name);
                let mut select_many_fn = Function::new(format!("{}_rocket_select_many", table.name));
                select_many_fn.arg(&alias_name, &alias_name)
                    .set_async(true)
                    .attr(&format!(
                        "rocket::get(\"/{}/by-ids/\", format=\"json\", data=\"<{}>\")",
                        table.name, alias_name,
                    ))
                    .ret("rocket::response::content::Json<String>")
                    .line("let mut client = create_client().await;")
                    .line(format!("let results = {}::select_many(&mut client, &params.data).await.unwrap();", table.name))
                    .line("rocket::response::content::Json(serde_json::to_string(&results).unwrap())");

                self.scope.push_fn(post_fn);
                self.scope.push_fn(get_fn);
                self.scope.push_fn(delete_fn);
                self.scope.push_fn(put_fn);
                //self.scope.push_fn(select_many_fn);
                
            }
        }
        Ok(self)
    }
    pub fn build(&self) -> String {
        self.scope.to_string()
    }
}
