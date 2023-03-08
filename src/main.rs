use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufRead;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    //let current_dir: PathBuf = env::current_dir()?;
    let path = Path::new("../syn/src");
    let number_regex = Regex::new(r#"\d+"#).unwrap();
    let struct_regex = Regex::new(r"struct").unwrap();
    let enum_regex = Regex::new(r"enum").unwrap();
    let array_regex = Regex::new(r"\[[^;\]]+;\s*\d+\]").unwrap();

    let dyn_vec = Regex::new(r"(vec!\[\])|(Vec<)|Vec::n").unwrap();
    let regexes = vec![number_regex, struct_regex, enum_regex, array_regex, dyn_vec];

    let mut results: HashMap<String, i32> = HashMap::new();
    let mut total_line_counter = 0;
    let mut total_number_of_files = 0;
    process_directory(
        path,
        &regexes,
        &mut results,
        &mut total_line_counter,
        &mut total_number_of_files,
    )?;
    println!("Hashmap : {results:?}");
    println!("Total lines : {total_line_counter}");
    println!("Total number of files : {total_number_of_files}");

    Ok(())
}

fn process_directory(
    path: &Path,
    regexes: &Vec<Regex>,
    results: &mut HashMap<String, i32>,
    total_line_counter: &mut i32,
    total_number_of_files: &mut i32,
) -> Result<(), anyhow::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        for r in regexes {
            if path.is_file() {
                *total_number_of_files += 1;
                count_in_file(&path, r, results, total_line_counter);
            } else if path.is_dir() {
                println!("Changing dir to {path:?}");
                process_directory(
                    &path,
                    regexes,
                    results,
                    total_line_counter,
                    total_number_of_files,
                )?;
            }
        }
    }
    Ok(())
}

fn count_in_file(
    path: &PathBuf,
    regex: &Regex,
    results: &mut HashMap<String, i32>,
    total_line_counter: &mut i32,
) {
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);
    reader.lines().flatten().for_each(|line| {
        *total_line_counter += 1;
        if regex.is_match(&line) {
            let key = regex.as_str().to_string();
            match results.get(&key) {
                Some(count) => results.insert(key, count + 1),
                None => results.insert(key, 1),
            };
        }
    });
}
