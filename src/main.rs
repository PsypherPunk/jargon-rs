use argh::FromArgs;
use colored::Colorize;
use rand::rng;
use rand::seq::IndexedRandom;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(FromArgs)]
/// Show an entry from the Jargon File.
struct Args {
    #[argh(switch, short = 'v')]
    /// display the current version
    version: bool,
}

fn main() {
    let args: Args = argh::from_env();
    if args.version {
        println!("{VERSION}");
    } else {
        let choice: &jargon::Jargon = jargon::JARGON.choose(&mut rng()).unwrap();
        println!("{}\n\n{}", choice.term.bold(), choice.definition.italic());
    }
}
