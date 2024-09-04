use clap::{Parser, CommandFactory};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

/// Simple program to parse C code and print variable types
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory containing C project files
    #[arg(short, long)]
    dir: String,

    /// Type to unroll
    #[arg(short, long)]
    typ: String,
}

fn main() {
    let args = Args::parse();

    // Collect all .c and .h files in the specified directory
    let mut files = vec![];
    for entry in WalkDir::new(&args.dir) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |s| s == "c" || s == "h") {
            files.push(path.to_path_buf());
        }
    }

    // Print found files for debugging
    println!("Found files: {:?}", files);

    // Read all files and store their contents
    let mut contents = String::new();
    for file in files {
        let content = fs::read_to_string(&file).unwrap_or_else(|err| {
            eprintln!("Failed to read file {}: {}", file.display(), err);
            Args::command().print_help().unwrap();
            std::process::exit(1);
        });
        contents.push_str(&content);
    }

    // Print content for debugging
    // println!("Combined file content:\n{}", contents);

    // Define regex patterns for typedef, struct, union, and enum
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

    // Print parsed typedefs, structs, unions, and enums for debugging
    // println!("Typedefs: {:?}", typedefs);
    // println!("Structs: {:?}", structs);
    // println!("Unions: {:?}", unions);
    // println!("Enums: {:?}", enums);

    // Unroll the specified type
    if let Some(unrolled) = unroll_type(&args.typ, &typedefs, &structs, &unions, &enums) {
        println!("{}", unrolled);
    } else {
        println!("Type {} not found", args.typ);
    }
}

fn unroll_type(
    typ: &str,
    typedefs: &HashMap<String, (String, String)>,
    structs: &HashMap<String, String>,
    unions: &HashMap<String, String>,
    enums: &HashMap<String, String>,
) -> Option<String> {
    let mut result = String::new();

    if let Some((kind, body)) = typedefs.get(typ) {
        result.push_str(&format!("Typedef: {} is an alias for {} {{ {} }}\n", typ, kind, body));
        if !body.is_empty() {
            return Some(result + &parse_fields(body, typedefs, structs, unions, enums));
        } else {
            return unroll_type(kind, typedefs, structs, unions, enums);
        }
    }
    if let Some(fields) = structs.get(typ) {
        result.push_str(&format!("Struct: {}\n", typ));
        result.push_str(&parse_fields(fields, typedefs, structs, unions, enums));
        return Some(result);
    }
    if let Some(fields) = unions.get(typ) {
        result.push_str(&format!("Union: {}\n", typ));
        result.push_str(&parse_fields(fields, typedefs, structs, unions, enums));
        return Some(result);
    }
    if let Some(constants) = enums.get(typ) {
        result.push_str(&format!("Enum: {}\n", typ));
        result.push_str(&parse_enum_constants(constants));
        return Some(result);
    }

    None
}

fn parse_fields(
    fields: &str,
    typedefs: &HashMap<String, (String, String)>,
    structs: &HashMap<String, String>,
    unions: &HashMap<String, String>,
    enums: &HashMap<String, String>,
) -> String {
    let field_re = Regex::new(r"(\w+)\s+(\w+);").unwrap();
    let mut result = String::new();

    for cap in field_re.captures_iter(fields) {
        let field_type = &cap[1];
        let field_name = &cap[2];
        result.push_str(&format!("  Field: {} of type {}\n", field_name, field_type));

        if let Some(unrolled) = unroll_type(field_type, typedefs, structs, unions, enums) {
            result.push_str(&unrolled);
        }
    }

    result
}

fn parse_enum_constants(constants: &str) -> String {
    let constant_re = Regex::new(r"(\w+)\s*(=\s*\d+)?\s*,?").unwrap();
    let mut result = String::new();

    for cap in constant_re.captures_iter(constants) {
        result.push_str(&format!("  Constant: {}\n", &cap[1]));
    }

    result
}
