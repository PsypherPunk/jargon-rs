use regex::Regex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("jargon.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let split = Regex::new(r"\s+_{106}\s+").unwrap();
    let mut jargon: Vec<&str> = split.split(contents.as_ref()).collect();

    let mut start = 0;
    let mut end = 0;
    for (i, entry) in jargon.iter().enumerate() {
        if entry.contains("Node:The Jargon Lexicon") {
            start = i + 1;
        }
        if entry.contains("Node:Appendix A") {
            end = i;
        }
    }

    jargon = jargon[start..end].to_vec();
    jargon = jargon
        .into_iter()
        .filter(|entry| !entry.contains("Node:=") && !entry.contains(r"\n="))
        .collect();
    let mut last = jargon[jargon.len() - 1].clone().to_owned();
    let last_position = jargon.len() - 1;
    last = last.replace("(Lexicon Entries End Here)", "");
    jargon[last_position] = last.as_str();

    let jargon = jargon
        .into_iter()
        .map(|entry| {
            entry
                .lines()
                .filter(|line| line.len() > 0 && !line.starts_with("Node:"))
                .map(|line| line.trim())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let mut lib_rs = OpenOptions::new()
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
    writeln!(lib_rs, "];")?;

    Ok(())
}
