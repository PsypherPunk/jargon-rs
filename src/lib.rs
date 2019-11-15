#[macro_use]
extern crate lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Jargon {
    pub term: String,
    pub definition: String,
}

const JARGON_JSON: &str = include_str!("resources/jargon.json");

lazy_static! {
    pub static ref JARGON: Vec<Jargon> = serde_json::from_str(&JARGON_JSON).unwrap();
}
