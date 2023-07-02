#[macro_use]
extern crate rocket;

use std::net::IpAddr;
use std::collections::HashMap;

use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct NewConnection {
    ip: IpAddr,
    headers: HashMap<String, Vec<String>>,
}

#[launch]
fn rocket() -> _ {
    use rocket::fairing::AdHoc;

    let builder = rocket::build();
        api::schema::mount_routes(builder).attach(AdHoc::on_request("Compatibility Normalizer", |req, _| Box::pin(async move {
            if !req.uri().is_normalized_nontrailing() {
                let normal = req.uri().clone().into_normalized_nontrailing();
                warn!("Incoming request URI was normalized for compatibility.");
                info!("{} -> {}", req.uri(), normal);
                req.set_uri(normal);
            }
        })))
}