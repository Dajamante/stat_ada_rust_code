use anyhow::Result;
use log::info;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufRead;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting application");

    let number_regex = Regex::new(r#"\b\d+\b"#).unwrap();
    let struct_regex = Regex::new(r"struct").unwrap();
    let enum_regex = Regex::new(r"enum").unwrap();
    let array_regex = Regex::new(r"\[.*;.*\]").unwrap();
    let ref_regex = Regex::new(r"&[mut\s]*\w+").unwrap();
    let heap_regex: Regex = Regex::new(r"Box<[^<>]+>|Rc<[^<>]+>|Arc<[^<>]+>").unwrap();
    //let unsafe_regex: Regex = Regex::new(r"unsafe \{").unwrap();

    let dyn_vec_regex = Regex::new(r"(vec!\[\])|(Vec<)|Vec::n").unwrap();
    let string_regex = Regex::new(r"String::|.to_string|format!\(").unwrap();
    let regexes = vec![
        number_regex,
        struct_regex,
        enum_regex,
        array_regex,
        dyn_vec_regex,
        string_regex,
        heap_regex,
        ref_regex,
    ];

    let path = Path::new("../famous_crates");

    let mut all_hashmaps_vec: Vec<HashMap<String, i32>> = Vec::new();

    for sub_dir in path.read_dir().expect("Could not access the famous dir") {
        let mut results: HashMap<String, i32> = HashMap::new();
        let mut total_line_counter = 0;
        let mut total_number_of_files = 0;
        let sub_dir = sub_dir.expect("Could not read subdirectory");
        let sub_dir_path = sub_dir.path();

        // For now let's avoid Ada
        if sub_dir_path.to_string_lossy().contains("ada") {
            continue;
        }
        process_directory(
            &sub_dir_path,
            &regexes,
            &mut results,
            &mut total_line_counter,
            &mut total_number_of_files,
        )
        .unwrap_or_else(|err| {
            eprintln!(
                "Error processing directory {}: {}",
                sub_dir_path.display(),
                err
            );
        });

        println!("Dir : {:#?} : {results:#?}", &sub_dir_path.display());

        let total: i32 = results.values().sum();
        println!("Number of total occurences : {total}");
        for (k, v) in &results {
            println!(
                "key : {k}, value : {v}, percentage {:.4}",
                (*v as f64 / total as f64)
            );
        }
        println!("Total LOC : {total_line_counter}");
        println!("Total number of files : {total_number_of_files}\n\n");

        all_hashmaps_vec.push(results);
    }

    let mut final_hashmap = HashMap::new();
    for hm in all_hashmaps_vec {
        for (k, v) in hm {
            match final_hashmap.get(&k) {
                Some(count) => final_hashmap.insert(k, v + count),
                None => final_hashmap.insert(k, v),
            };
        }
    }

    println!("Final result: {final_hashmap:?}");

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

        if path.is_file() {
            *total_number_of_files += 1;
            for r in regexes {
                count_in_file(&path, r, results, total_line_counter);
            }
        } else if path.is_dir() {
            process_directory(
                &path,
                regexes,
                results,
                total_line_counter,
                total_number_of_files,
            )?;
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
            let key = create_key(regex);
            match results.get(&key) {
                Some(count) => results.insert(key, count + 1),
                None => results.insert(key, 1),
            };
        }
    });
}

fn create_key(regex: &Regex) -> String {
    let key = match regex.as_str() {
        "\\[.*;.*\\]" => "Array".to_string(),
        "Box<[^<>]+>|Rc<[^<>]+>|Arc<[^<>]+>" => "Box/Rc/Arc".to_string(),
        "&[mut\\s]*\\w+" => "&T or &mut T".to_string(),
        "String::|.to_string|format!\\(" => "String".to_string(),
        "(vec!\\[\\])|(Vec<)|Vec::n" => "Vec".to_string(),
        "\\b\\d+\\b" => "Number".to_string(),
        "struct" => "Struct".to_string(),
        "enum" => "Enum".to_string(),
        // this is a kind of error I guess
        _ => String::from("Not a regex"),
    };
    key
}
