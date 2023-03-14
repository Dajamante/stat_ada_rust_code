use regex::Regex;
pub fn rust_regexes() -> Vec<Regex> {
    let number_regex = Regex::new(r#"\b\d+\b"#).unwrap();
    let struct_regex = Regex::new(r"struct").unwrap();
    let enum_regex = Regex::new(r"enum").unwrap();
    let array_regex = Regex::new(r"\[.*;.*\]").unwrap();
    let ref_regex = Regex::new(r"&[mut\s]*\w+").unwrap();
    let heap_regex: Regex = Regex::new(r"Box<[^<>]+>|Rc<[^<>]+>|Arc<[^<>]+>").unwrap();
    // Counts of the unsafe keyword can be useful later.
    //let unsafe_regex: Regex = Regex::new(r"unsafe \{").unwrap();

    let dyn_vec_regex = Regex::new(r"(vec!\[\])|(Vec<)|Vec::n").unwrap();
    let string_regex = Regex::new(r"String::|.to_string|format!\(").unwrap();
    vec![
        number_regex,
        struct_regex,
        enum_regex,
        array_regex,
        dyn_vec_regex,
        string_regex,
        heap_regex,
        ref_regex,
    ]
}

pub fn spark_regexes() -> Vec<Regex> {
    let number_regex = Regex::new(r#"\b\d+\b"#).unwrap();
    let struct_regex = Regex::new("record").unwrap();
    let enum_regex = Regex::new(r#"type\s{1}[a-zA-Z_]+\s{1}is\s{1}\("#).unwrap();
    let array_regex = Regex::new("is array").unwrap();
    // new is used for memory allocation but access is sufficient
    let ref_regex = Regex::new(r"\b(access)\s+(\w+)\b").unwrap();
    // Counts of the unsafe keyword can be useful later.
    //let unsafe_regex: Regex = Regex::new(r"unsafe \{").unwrap();
    vec![
        number_regex,
        struct_regex,
        enum_regex,
        array_regex,
        ref_regex,
    ]
}
