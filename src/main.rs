///! This program reads through a directory and its
/// subdirectories, looking for certain patterns using
/// regular expressions.
/// Specifically, it counts the occurrences of different
/// patterns such as numbers, arrays, references, strings,
/// and various heap-allocated types.
///
/// The program creates a list of regular expressions to use
/// for pattern matching, then loops through each subdirector
///  in the specified directory, processing each file in the
/// directory and its subdirectories.
///
/// For each file, it counts the occurrences of each pattern
/// using the specified regular expressions, then adds the counts
///  to a hashmap. The program also tracks the total number of
///  lines and files processed.
///
/// Once all files have been processed, the program adds up the counts
///  from all hashmaps and creates a final hashmap with the total
///  counts for each pattern. Finally, the program prints out the final
/// hashmap to the console.
///
/// Call it with `cargo run -- --code-base rust/ada
use anyhow::Result;
use clap::{arg, command, Parser};
use log::info;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::BufRead;
use std::io::BufReader;
use std::path::{Path, PathBuf};

mod regexes_mod;
use crate::regexes_mod::*;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short = 'c', long)]
    code_base: String,
}

trait StatPrinter {
    fn print(&self);
}

impl StatPrinter for HashMap<String, i32> {
    fn print(&self) {
        let total: i32 = self.values().sum();
        println!("Total number of types : {total}");
        for (k, v) in self {
            println!(
                "key : {k}, value : {v}, percentage {:.4}",
                (*v as f64 / total as f64)
            );
        }
    }
}

impl StatPrinter for HashMap<String, HashMap<String, i32>> {
    fn print(&self) {
        for v in self.values() {
            v.print();
        }
    }
}
fn main() -> Result<()> {
    env_logger::init();
    info!("Starting application");

    let args = Args::parse();
    let code_base = args.code_base;
    let mut grand_total_files: i32 = 0;
    let mut grand_total_loc: i32 = 0;

    let (path, regexes): (&Path, Vec<Regex>) = match &code_base {
        p if p == "rust" => (Path::new("../famous_crates/rust"), rust_regexes()),
        p if p == "spark" => (Path::new("../famous_crates/ada"), spark_regexes()),
        &_ => return Err(anyhow::anyhow!("Invalid code_base argument")),
    };
    // Define the name command line option

    let mut all_hashmaps: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for sub_dir in path.read_dir().expect("Could not access the famous dir") {
        let mut results: HashMap<String, i32> = HashMap::new();
        let mut total_line_counter = 0;
        let mut total_number_of_files = 0;
        let sub_dir = sub_dir.expect("Could not read subdirectory");
        let sub_dir_path = sub_dir.path();

        process_directory(
            &sub_dir_path,
            &regexes,
            &mut results,
            &mut total_line_counter,
            &mut total_number_of_files,
            &mut grand_total_loc,
            &mut grand_total_files,
        )
        .unwrap_or_else(|err| {
            eprintln!(
                "Error processing directory {}: {}",
                sub_dir_path.display(),
                err
            );
        });

        print_stat(
            &sub_dir_path,
            &results,
            total_number_of_files,
            total_line_counter,
        );

        all_hashmaps.insert(
            sub_dir_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            results,
        );
    }

    let mut final_hashmap: HashMap<String, i32> = HashMap::new();
    for hm in all_hashmaps.values() {
        for (k, v) in hm {
            match final_hashmap.get(k) {
                Some(count) => final_hashmap.insert(k.to_string(), v + count),
                None => final_hashmap.insert(k.to_string(), *v),
            };
        }
    }
    print_latex_output(&all_hashmaps);
    println!("Final result: {final_hashmap:?}");
    println!("Grand total LOC : {grand_total_files}");
    println!("Total number of files : {grand_total_loc}\n\n");
    print_stat(path, &final_hashmap, grand_total_files, grand_total_loc);

    Ok(())
}

fn print_stat<T: StatPrinter + std::fmt::Debug>(
    sub_dir_path: &Path,
    results: &T,
    files: i32,
    lines: i32,
) {
    println!("Dir : {:#?} : {:#?}", &sub_dir_path.display(), results);
    println!("CALLING STAT PRINTER:");
    results.print();
    println!("Total LOC : {lines}");
    println!("Total number of files : {files}\n\n");
}

fn process_directory(
    path: &Path,
    regexes: &Vec<Regex>,
    results: &mut HashMap<String, i32>,
    total_line_counter: &mut i32,
    total_number_of_files: &mut i32,
    grand_total_files: &mut i32,
    grand_total_loc: &mut i32,
) -> Result<(), anyhow::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;

        let path = entry.path();

        if path.is_file() {
            *total_number_of_files += 1;
            *grand_total_files += 1;
            for r in regexes {
                count_in_file(&path, r, results, total_line_counter, grand_total_loc);
            }
        } else if path.is_dir() {
            process_directory(
                &path,
                regexes,
                results,
                total_line_counter,
                total_number_of_files,
                grand_total_files,
                grand_total_loc,
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
    grand_total_loc: &mut i32,
) {
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);
    reader.lines().flatten().for_each(|line| {
        *total_line_counter += 1;
        *grand_total_loc += 1;
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
        r#"\[(\s*\d+\s*(?:,\s*\d+\s*)*)\]"# | r#"type [a-zA-Z_]+ is array\(\d.."# => {
            "Array".to_string()
        }
        "Box<[^<>]+>|Rc<[^<>]+>|Arc<[^<>]+>" => "Box/Rc/Arc".to_string(),
        "&[mut\\s]*\\w+" | r"type\s{1}[a-zA-Z_]+\s{1}is\s{1}array" => "&T or &mut T".to_string(),
        "String::|.to_string|format!\\(" => "String".to_string(),
        "vec!|Vec::n" => "Vec".to_string(),
        r#"\b(?:\d+(?:\.\d+)?|\d+(?:_\d+)+)\b"#
        | r#"Integer|Float|Fixed|Decimal|Modular|Natural|Positive|Long|range \d .. \d"# => {
            "Number".to_string()
        }
        "struct" => "Struct".to_string(),
        "enum" | r"type\s{1}[a-zA-Z_]+\s{1}is\s{1}\(" => "Enum".to_string(),
        "is record" => "Record".to_string(),
        r"access|new" => "Access".to_string(),
        r#"range <>|Containers.Vector"# => "Unconstrained array".to_string(),
        "Strings.Unbounded" => "Unbounded strings".to_string(),
        // this is a kind of error I guess
        els => format!("Not a valid regex {els}"),
    };
    key
}

fn print_latex_output(all_hashmaps: &HashMap<String, HashMap<String, i32>>) {
    println!(r"\pgfplotstableread{{");
    println!("Rust_type crates_io lib_rs rustc");

    let rust_types = [
        "Number",
        "&T or &mut T",
        "Array",
        "Vec",
        "Enum",
        "Struct",
        "String",
    ];
    let mut dir_totals: HashMap<String, i32> = HashMap::new();
    for (dir, hm) in all_hashmaps {
        dir_totals.insert(dir.clone(), hm.values().sum());
    }

    for rt in &rust_types {
        print!("{} ", rt);
        let mut sorted_dirs: Vec<_> = all_hashmaps.keys().collect();
        sorted_dirs.sort();
        for dir in all_hashmaps.keys() {
            let count = match dir.as_str() {
                "rustc" | "cratesio" | "librs" => all_hashmaps
                    .get(dir)
                    .and_then(|hm| hm.get(*rt))
                    .unwrap_or(&0),
                _ => continue,
            };

            let total = dir_totals.get(dir).unwrap_or(&0);
            let percentage = (*count as f64 / *total as f64) * 100.0;
            print!(" {} ({:.2}%)", count, percentage);
        }
        println!();
    }

    println!("}}\\EXPDATA");
}
