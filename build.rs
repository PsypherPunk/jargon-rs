use regex::Regex;
use std::fs;
use std::io::prelude::*;

use flate2::read::GzDecoder;

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
                .filter(|line| !line.is_empty() && !line.starts_with("Node:"))
                .map(str::trim)
                .collect()
        })
        .collect()
}

/// Return the subsection of entries containing Jargon definitions.
///
/// The Jargon file contains a number of sections beyond the
/// dictionary entries—strip out those extraneous sections.
fn get_relevant_entries<'a>(entries: &'a [&'a str]) -> Vec<&'a str> {
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
        .iter()
        .copied()
        .filter(|entry| !entry.contains("Node:=") && !entry.contains(r"\n="))
        .collect()
}

/// Read in the `jargon.txt.gz` file.
///
/// Additionally, remove the closing note indicating the end of jargon entries.
fn get_jargon() -> String {
    let compressed = fs::read("jargon.txt.gz").expect("Error reading jargon.txt.gz");
    let mut gz = GzDecoder::new(&compressed[..]);
    let mut contents = String::new();
    gz.read_to_string(&mut contents).unwrap();

    contents.replace("(Lexicon Entries End Here)", "")
}

fn write_lib(jargon: Vec<Vec<&str>>) -> std::io::Result<()> {
    let mut lib_rs = fs::OpenOptions::new()
        .write(true)
        .append(false)
        .open("src/lib.rs")
        .unwrap();

    write!(
        lib_rs,
        "pub struct Jargon {{\n    pub term: &'static str,\n    pub definition: &'static str,\n}}\n\n",
    )?;

    let open = format!(
        "#[allow(clippy::needless_raw_string_hashes, clippy::large_const_arrays)]\npub const JARGON: [Jargon; {}] = [",
        jargon.len()
    );
    writeln!(lib_rs, "{open}")?;
    for entry in jargon {
        let term = entry.first().unwrap();
        let definition = entry[1..].join(" ");
        let row = format!(
            "    Jargon {{\n        term: r##\"{term}\"##,\n        definition: r##\"{definition}\"##,\n    }},",
        );
        writeln!(lib_rs, "{row}")?;
    }

    writeln!(lib_rs, "];")
}

fn main() {
    let contents = get_jargon();
    let split = Regex::new(r"\s+_{131}\s+").unwrap();
    let splits = split.split(contents.as_ref()).collect::<Vec<&str>>();
    let entries: Vec<&str> = get_relevant_entries(&splits);
    let jargon = parse_entries(entries);

    write_lib(jargon).expect("Unable to write lib.rs");
}
