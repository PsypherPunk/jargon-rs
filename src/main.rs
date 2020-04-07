extern crate jargon;

use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let choice: &jargon::Jargon = jargon::JARGON.choose(&mut thread_rng()).unwrap();
    println!("{}\n\n{}", choice.term.bold(), choice.definition.italic());
}
