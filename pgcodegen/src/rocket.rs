use crate::Table;
use codegen::Function;
use codegen::Scope;
pub struct RocketBuilder {
    tables: Vec<Table>,
}

impl RocketBuilder {
    pub fn new(tables: Vec<Table>) -> Self {
        Self { tables }
    }

    pub fn generate(&self, scope: &mut Scope) {
        for table in self.tables.iter() {
            let mut get_fn = Function::new(&format!("{}_rocket_get", table.name));
            let mut post_fn = Function::new(&format!("{}_rocket_post", table.name));
            let mut put_fn = Function::new(&format!("{}_rocket_put", table.name));
            let mut delete_fn = Function::new(&format!("{}_rocket_delete", table.name));
        }
    }
}
