use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::{LOG_ERR, log_level};

pub struct ParsedTypes {
    pub typedefs: HashMap<String, (String, String)>,
    pub structs: HashMap<String, String>,
    pub unions: HashMap<String, String>,
    pub enums: HashMap<String, String>,
}

pub fn parse_files(files: Vec<PathBuf>) -> ParsedTypes {
    let contents: Vec<String> = files.into_par_iter()
        .map(|file| fs::read_to_string(&file).unwrap_or_else(|err| {
            LOG_ERR!("Failed to read file {}: {}", file.display(), err);
            String::new()
        }))
        .collect();

    let typedef_re = Regex::new(r"typedef\s+(struct|union|enum)?\s*\{?([^}]*)\}?\s*(\w+);").unwrap();
    let struct_re = Regex::new(r"struct\s+(\w+)\s*\{([^}]*)\};").unwrap();
    let union_re = Regex::new(r"union\s+(\w+)\s*\{([^}]*)\};").unwrap();
    let enum_re = Regex::new(r"enum\s+(\w+)\s*\{([^}]*)\};").unwrap();

    let parsed_types = contents.into_par_iter().fold(
        || ParsedTypes { // Initial value for each thread
            typedefs: HashMap::new(),
            structs: HashMap::new(),
            unions: HashMap::new(),
            enums: HashMap::new(),
        },
        |mut acc, content| { // Accumulation function
            for cap in typedef_re.captures_iter(&content) {
                let typedef_kind = cap.get(1).map_or("", |m| m.as_str()).to_string();
                let typedef_body = cap.get(2).map_or("", |m| m.as_str()).to_string();
                let typedef_name = &cap[3];
                acc.typedefs.insert(typedef_name.to_string(), (typedef_kind, typedef_body));
            }
            for cap in struct_re.captures_iter(&content) {
                acc.structs.insert(cap[1].to_string(), cap[2].to_string());
            }
            for cap in union_re.captures_iter(&content) {
                acc.unions.insert(cap[1].to_string(), cap[2].to_string());
            }
            for cap in enum_re.captures_iter(&content) {
                acc.enums.insert(cap[1].to_string(), cap[2].to_string());
            }
            acc // Return the accumulated value
        },
    ).reduce(|| ParsedTypes { // Reduction function to combine results
        typedefs: HashMap::new(),
        structs: HashMap::new(),
        unions: HashMap::new(),
        enums: HashMap::new(),
    }, |mut a, b| {
        a.typedefs.extend(b.typedefs);
        a.structs.extend(b.structs);
        a.unions.extend(b.unions);
        a.enums.extend(b.enums);
        a
    });

    parsed_types
}
