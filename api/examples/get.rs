#[macro_use]
extern crate lazy_static;

use api::schema::*;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

// provides an example of a fully formatted get route
lazy_static! {
    static ref CLIENT: Arc<Mutex<postgres::Client>> = {
        Arc::new(Mutex::new(
            postgres::Client::connect(
                "postgresql://cardinal:Qksg0FV2EMDM@192.168.122.1:5432/myhealth",
                postgres::NoTls,
            )
            .unwrap(),
        ))
    };
}

#[rocket::get("/eventinstances/<id>", format = "json")]
fn eventinstances_rocket_get(id: uuid::Uuid) -> rocket::response::content::RawJson<String> {
    let mut client = CLIENT.lock().unwrap();
    rocket::response::content::RawJson(
        serde_json::to_string(
            &eventinstances::select(client.deref_mut(), &id)
                .unwrap()
                .unwrap(),
        )
        .unwrap(),
    )
}

fn main() {}
