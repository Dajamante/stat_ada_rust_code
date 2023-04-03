use regex::Regex;
pub fn rust_regexes() -> Vec<Regex> {
    let number_regex = Regex::new(r#"\b(?:\d+(?:\.\d+)?|\d+(?:_\d+)+)\b"#).unwrap();
    let struct_regex = Regex::new(r"struct").unwrap();
    let enum_regex = Regex::new(r"enum").unwrap();
    let array_regex = Regex::new(r#"\[(\s*\d+\s*(?:,\s*\d+\s*)*)\]"#).unwrap();
    let ref_regex = Regex::new(r"&[mut\s]*\w+").unwrap();
    let heap_regex: Regex = Regex::new(r"Box<[^<>]+>|Rc<[^<>]+>|Arc<[^<>]+>").unwrap();
    // Counts of the unsafe keyword can be useful later.
    //let unsafe_regex: Regex = Regex::new(r"unsafe \{").unwrap();

    let dyn_vec_regex = Regex::new(r"vec!|Vec::new").unwrap();
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
    //let number_regex = Regex::new(r#"\b\d+\b"#).unwrap();
    let number_regex = Regex::new("(:(\\s*)(Integer|Float|Fixed|Decimal|Modular|Natural|Positive|Long|Short))|(range(\\s*)(-)?\\d+)|(is(\\s*)digits)").unwrap();
    let struct_regex = Regex::new("is record").unwrap();
    let enum_regex = Regex::new(r#"type\s{1}[a-zA-Z_]+\s{1}is\s{1}\("#).unwrap();
    let array_regex = Regex::new(r#"type [a-zA-Z_]+ is array\(\d.."#).unwrap();
    // new is used for memory allocation but access is sufficient
    let ref_regex = Regex::new(r"access|new").unwrap();
    let unconstrained_regex = Regex::new(r#"range <>|Containers.Vector"#).unwrap();
    let unbounded_regex: Regex = Regex::new(r"Strings.Unbounded").unwrap();
    vec![
        number_regex,
        struct_regex,
        enum_regex,
        array_regex,
        ref_regex,
        unconstrained_regex,
        unbounded_regex,
    ]
}
