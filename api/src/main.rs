#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::net::IpAddr;

use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConnection {
    ip: IpAddr,
    headers: HashMap<String, Vec<String>>,
}

#[launch]
fn rocket() -> _ {
    use rocket::fairing::AdHoc;

    api::schema::mount_routes(rocket::build())
    .attach(AdHoc::on_request(
        "Compatibility Normalizer",
        |req, _| {
            Box::pin(async move {
                if !req.uri().is_normalized_nontrailing() {
                    let normal = req.uri().clone().into_normalized_nontrailing();
                    warn!("Incoming request URI was normalized for compatibility.");
                    info!("{} -> {}", req.uri(), normal);
                    req.set_uri(normal);
                }
            })
        },
    ))
}
