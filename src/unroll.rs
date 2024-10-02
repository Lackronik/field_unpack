use crate::parser::ParsedTypes;

pub fn unroll_type(
    typ: &str,
    parsed_types: &ParsedTypes,
) -> Option<String> {
    let mut result = String::new();

    if let Some((kind, body)) = parsed_types.typedefs.get(typ) {
        result.push_str(&format!("Typedef: {} is an alias for {} {{ {} }}\n", typ, kind, body));
        if !body.is_empty() {
            return Some(result + &parse_fields(body, parsed_types));
        } else {
            return unroll_type(kind, parsed_types);
        }
    }
    if let Some(fields) = parsed_types.structs.get(typ) {
        result.push_str(&format!("Struct: {}\n", typ));
        result.push_str(&parse_fields(fields, parsed_types));
        return Some(result);
    }
    if let Some(fields) = parsed_types.unions.get(typ) {
        result.push_str(&format!("Union: {}\n", typ));
        result.push_str(&parse_fields(fields, parsed_types));
        return Some(result);
    }
    if let Some(constants) = parsed_types.enums.get(typ) {
        result.push_str(&format!("Enum: {}\n", typ));
        result.push_str(&parse_enum_constants(constants));
        return Some(result);
    }

    None
}

fn parse_fields(fields: &str, parsed_types: &ParsedTypes) -> String {
    let field_re = regex::Regex::new(r"(\w+)\s+(\w+);").unwrap();
    let mut result = String::new();

    for cap in field_re.captures_iter(fields) {
        let field_type = &cap[1];
        let field_name = &cap[2];
        result.push_str(&format!("  Field: {} of type {}\n", field_name, field_type));

        if let Some(unrolled) = unroll_type(field_type, parsed_types) {
            result.push_str(&unrolled);
        }
    }

    result
}

fn parse_enum_constants(constants: &str) -> String {
    let constant_re = regex::Regex::new(r"(\w+)\s*(=\s*\d+)?\s*,?").unwrap();
    let mut result = String::new();

    for cap in constant_re.captures_iter(constants) {
        result.push_str(&format!("  Constant: {}\n", &cap[1]));
    }

    result
}
