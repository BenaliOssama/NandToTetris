use super::*;
use std::collections::HashMap;

pub fn process(contents: String) -> Vec<String> {
    let mut results = Vec::<String>::new();
    let mut labeled = Vec::<String>::new();

    let mut line_number: u32 = 0;
    let mut labels = HashMap::<String, u16>::new();

    // R0 to R15
    for i in 0..16 {
        let name = format!("R{}", i);
        labels.insert(name, i);
    }

    // Predefined symbols
    labels.insert(String::from("SP"), 0);
    labels.insert(String::from("LCL"), 1);
    labels.insert(String::from("ARG"), 2);
    labels.insert(String::from("THIS"), 3);
    labels.insert(String::from("THAT"), 4);
    labels.insert(String::from("SCREEN"), 16384);
    labels.insert(String::from("KBD"), 24576);

    // First pass: collect labels
    for line in contents.lines() {
        if line.starts_with('(') && line.ends_with(')') {
            // Get label without parentheses
            let value = &line[1..line.len() - 1];
            labels.insert(String::from(value), line_number as u16);
            continue;
        }
        line_number += 1;
        labeled.push(line.to_string());
    }

    let mut next_variable_addr = 16; // Variables start at R16

    // Second pass: assemble instructions
    for line in labeled {
        let assembled = if line.starts_with('@') {
            a_instruction(&line, &mut labels, &mut next_variable_addr)
        } else {
            c_instruction(&line)
        };
        results.push(assembled);
    }

    results
}

fn a_instruction(
    input: &str,
    labels: &mut HashMap<String, u16>,
    next_variable_addr: &mut u16
) -> String {
    let variable = input.trim_start_matches('@');

    let value: u16 = if is_valid_variable_name(variable) {
        if !labels.contains_key(variable) {
            let addr = *next_variable_addr;
            *next_variable_addr += 1;
            labels.insert(String::from(variable), addr);
        }
        *labels.get(variable).unwrap()
    } else if let Ok(num) = variable.parse::<u16>() {
        num
    } else {
        panic!("Invalid variable name or number: {}", variable);
    };

    format!("{:016b}", value)
}

fn c_instruction(c: &str) -> String {
    let (dest, comp, jump) = parse_c_instruction(c);

    let dest_map = get_dest_table();
    let comp_map = get_comp_table();
    let jump_map = get_jump_table();

    let dest_bin = dest_map.get(dest.as_str()).expect("Invalid dest");
    let comp_bin = comp_map.get(comp.as_str()).expect("Invalid comp");
    let jump_bin = jump_map.get(jump.as_str()).expect("Invalid jump");

    format!("111{}{}{}", comp_bin, dest_bin, jump_bin)
}

fn parse_c_instruction(line: &str) -> (String, String, String) {
    let mut dest = "";
    let mut comp = "";
    let mut jump = "";

    let mut comp_jump = line;

    if let Some(eq_pos) = line.find('=') {
        dest = &line[..eq_pos];
        comp_jump = &line[eq_pos + 1..];
    }

    if let Some(semi_pos) = comp_jump.find(';') {
        comp = &comp_jump[..semi_pos];
        jump = &comp_jump[semi_pos + 1..];
    } else {
        comp = comp_jump;
    }

    (dest.to_string(), comp.to_string(), jump.to_string())
}

fn is_valid_variable_name(s: &str) -> bool {
    let mut chars = s.chars();

    let _ = match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => c,
        _ => {
            return false;
        }
    };

    chars.all(|c| (c.is_ascii_alphanumeric() || c == '_'))
}
