use api::schema::*;
use postgres::{Client, NoTls};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
fn main() {
    let mut file = File::open(".dburl").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    let len = content.len();
    content.truncate(len - 1);
    let mut data: calldata = calldata::default();
    let id = uuid::Uuid::from_str("c3c9234a-8c8b-4470-829e-eea22cae2195").unwrap();
    let mut client = Client::connect(&content, NoTls).unwrap();
    let newdata = calldata::select(&mut client, &id).unwrap();
    println!("{:?}", newdata);
    newdata.unwrap().delete(&mut client).unwrap();
}
