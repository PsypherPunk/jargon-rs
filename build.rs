use regex::Regex;
use std::fs;
use std::io::prelude::*;

/// Transform each block of text into a collection of lines.
///
/// The first line corresponds to the term, the remainder to the
/// definition.
fn parse_entries(entries: Vec<&str>) -> Vec<Vec<&str>> {
    entries
        .into_iter()
        .map(|entry| {
            entry
                .lines()
                .filter(|line| line.len() > 0 && !line.starts_with("Node:"))
                .map(|line| line.trim())
                .collect()
        })
        .collect()
}

/// Return the subsection of entries containing Jargon definitions.
///
/// The Jargon file contains a number of sections beyond the
/// dictionary entries—strip out those extraneous sections.
fn get_relevant_entries(entries: Vec<&str>) -> Vec<&str> {
    let mut start = 0;
    let mut end = 0;

    for (i, entry) in entries.iter().enumerate() {
        if entry.contains("Node:The Jargon Lexicon") {
            start = i + 1;
        }
        if entry.contains("Node:Appendix A") {
            end = i;
        }
    }

    entries[start..end]
        .to_vec()
        .into_iter()
        .filter(|entry| !entry.contains("Node:=") && !entry.contains(r"\n="))
        .collect()
}

/// Read in the `jargon.txt` file.
///
/// Additionally, remove the closing note indicating the end of jargon entries.
fn get_jargon() -> String {
    let contents = fs::read_to_string("jargon.txt").expect("Error reading jargon.txt");
    print!("{}", contents);

    contents.replace("(Lexicon Entries End Here)", "")
}

fn write_lib(jargon: Vec<Vec<&str>>) -> std::io::Result<()> {
    let mut lib_rs = fs::OpenOptions::new()
        .write(true)
        .append(false)
        .open("src/lib.rs")
        .unwrap();

    write!(
        lib_rs, "{}",
        "pub struct Jargon {\n    pub term: &'static str,\n    pub definition: &'static str,\n}\n\n",
    )?;

    let open = format!("pub const JARGON: [Jargon; {}] = [", jargon.len());
    writeln!(lib_rs, "{}", open)?;
    for entry in jargon {
        let term = entry.get(0).unwrap();
        let definition = entry[1..].join(" ");
        let row = format!(
            "    Jargon {{\n        term: r##\"{}\"##,\n        definition: r##\"{}\"##,\n    }},\n",
            term,
            definition,
        );
        writeln!(lib_rs, "{}", row)?;
    }

    writeln!(lib_rs, "];")
}

fn main() {
    let contents = get_jargon();
    let split = Regex::new(r"\s+_{106}\s+").unwrap();
    let entries: Vec<&str> = get_relevant_entries(split.split(contents.as_ref()).collect());
    let jargon = parse_entries(entries);

    write_lib(jargon).expect("Unable to write lib.rs");
}
