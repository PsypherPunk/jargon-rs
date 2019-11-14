#[macro_use]
extern crate lazy_static;
use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Jargon {
    term: String,
    definition: String,
}

const JARGON_JSON: &str = include_str!("resources/jargon.json");

lazy_static! {
    static ref JARGON: Vec<Jargon> = serde_json::from_str(&JARGON_JSON).unwrap();
}

fn main() {
    let choice: &Jargon = JARGON.choose(&mut thread_rng()).unwrap();
    println!("{}\n\n{}", choice.term.bold(), choice.definition.italic());
}
