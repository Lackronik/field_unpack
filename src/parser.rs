use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct ParsedTypes {
    pub typedefs: HashMap<String, (String, String)>,
    pub structs: HashMap<String, String>,
    pub unions: HashMap<String, String>,
    pub enums: HashMap<String, String>,
}

pub fn parse_files(files: Vec<PathBuf>) -> ParsedTypes {
    let mut contents = String::new();
    for file in files {
        let content = fs::read_to_string(&file).unwrap_or_else(|err| {
            eprintln!("Failed to read file {}: {}", file.display(), err);
            std::process::exit(1);
        });
        contents.push_str(&content);
    }

    // Define regex patterns
    let typedef_re = Regex::new(r"typedef\s+(struct|union|enum)?\s*\{?([^}]*)\}?\s*(\w+);").unwrap();
    let struct_re = Regex::new(r"struct\s+(\w+)\s*\{([^}]*)\};").unwrap();
    let union_re = Regex::new(r"union\s+(\w+)\s*\{([^}]*)\};").unwrap();
    let enum_re = Regex::new(r"enum\s+(\w+)\s*\{([^}]*)\};").unwrap();

    // Parse and store all typedefs, structs, unions, and enums
    let mut typedefs = HashMap::new();
    let mut structs = HashMap::new();
    let mut unions = HashMap::new();
    let mut enums = HashMap::new();

    for cap in typedef_re.captures_iter(&contents) {
        let typedef_kind = cap.get(1).map_or("", |m| m.as_str()).to_string();
        let typedef_body = cap.get(2).map_or("", |m| m.as_str()).to_string();
        let typedef_name = &cap[3];
        typedefs.insert(typedef_name.to_string(), (typedef_kind, typedef_body));
    }
    for cap in struct_re.captures_iter(&contents) {
        structs.insert(cap[1].to_string(), cap[2].to_string());
    }
    for cap in union_re.captures_iter(&contents) {
        unions.insert(cap[1].to_string(), cap[2].to_string());
    }
    for cap in enum_re.captures_iter(&contents) {
        enums.insert(cap[1].to_string(), cap[2].to_string());
    }

    ParsedTypes {
        typedefs,
        structs,
        unions,
        enums,
    }
}
