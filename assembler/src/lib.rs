use std::error::Error;
use std::fs;
use std::io::{ BufWriter, Write };
use std::collections::HashMap;



fn get_dest_table() -> HashMap<&'static str, &'static str> {
    let mut dest = HashMap::new();
    dest.insert("",    "000");
    dest.insert("M",   "001");
    dest.insert("D",   "010");
    dest.insert("MD",  "011");
    dest.insert("A",   "100");
    dest.insert("AM",  "101");
    dest.insert("AD",  "110");
    dest.insert("AMD", "111");
    dest
}

fn get_comp_table() -> HashMap<&'static str, &'static str> {
    let mut comp = HashMap::new();
    comp.insert("0",   "0101010");
    comp.insert("1",   "0111111");
    comp.insert("-1",  "0111010");
    comp.insert("D",   "0001100");
    comp.insert("A",   "0110000");
    comp.insert("!D",  "0001101");
    comp.insert("!A",  "0110001");
    comp.insert("-D",  "0001111");
    comp.insert("-A",  "0110011");
    comp.insert("D+1", "0011111");
    comp.insert("A+1", "0110111");
    comp.insert("D-1", "0001110");
    comp.insert("A-1", "0110010");
    comp.insert("D+A", "0000010");
    comp.insert("D-A", "0010011");
    comp.insert("A-D", "0000111");
    comp.insert("D&A", "0000000");
    comp.insert("D|A", "0010101");
    comp.insert("M",   "1110000");
    comp.insert("!M",  "1110001");
    comp.insert("-M",  "1110011");
    comp.insert("M+1", "1110111");
    comp.insert("M-1", "1110010");
    comp.insert("D+M", "1000010");
    comp.insert("D-M", "1010011");
    comp.insert("M-D", "1000111");
    comp.insert("D&M", "1000000");
    comp.insert("D|M", "1010101");
    comp
}

fn get_jump_table() -> HashMap<&'static str, &'static str> {
    let mut jump = HashMap::new();
    jump.insert("",    "000");
    jump.insert("JGT", "001");
    jump.insert("JEQ", "010");
    jump.insert("JGE", "011");
    jump.insert("JLT", "100");
    jump.insert("JNE", "101");
    jump.insert("JLE", "110");
    jump.insert("JMP", "111");
    jump
}


pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("missing file name");
        }
        if args.len() > 2 {
            return Err("too many arguments!");
        }

        let file_path = args[1].clone();

        Ok(Config {
            file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Open output file for writing
    let output_file = fs::File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);

    let contents = fs::read_to_string(config.file_path)?;

    let results = &process(&contents);

    for line in results {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}


/*
A instruction @value 
C instruction dest = comp ; jmp 
white space 
*/

pub fn process( contents: &str) -> Vec<String> {
    let mut results = Vec::new();

    for line in contents.lines() {
        let trimmed = line.trim();
    let mut assembled = String::new(); 
        
        // instruction A
        if trimmed.starts_with("@"){
            assembled = a_instruction(&trimmed);
        }else{
            assembled = c_instruction(&trimmed);
        } 
        results.push(String::from(assembled)); 
    }
    
    results
}

fn a_instruction(input: &str) -> String {
    // translate a instructions
    
    // Remove the '@' and parse to number
    let num: u16 = input.trim_start_matches('@').parse().unwrap();

    // Format as 16-bit binary string, padded with 0s
    let binary = format!("{:016b}", num);     
    binary
}



fn c_instruction(c: &str) -> String {
    // translate c instructions
    let (dest, comp, jump) = parse_c_instruction(c);

    let dest_map = get_dest_table();
    let comp_map = get_comp_table();
    let jump_map = get_jump_table();


    // get binary parts, or panic if invalid
    let dest_bin = dest_map.get(dest.as_str()).expect("Invalid dest");
    let comp_bin = comp_map.get(comp.as_str()).expect("Invalid comp");
    let jump_bin = jump_map.get(jump.as_str()).expect("Invalid jump");

    // Build the final 16-bit instruction: "111" + comp + dest + jump
    format!("111{}{}{}", comp_bin, dest_bin, jump_bin)

}

fn parse_c_instruction(line: &str) -> (String, String, String) {
    let mut dest = "";
    let mut comp = "";
    let mut jump = "";

    let mut comp_jump = line;

    // Split dest
    if let Some(eq_pos) = line.find('=') {
        dest = &line[..eq_pos];
        comp_jump = &line[eq_pos + 1..];
    }

    // Split jump
    if let Some(semi_pos) = comp_jump.find(';') {
        comp = &comp_jump[..semi_pos];
        jump = &comp_jump[semi_pos + 1..];
    } else {
        comp = comp_jump;
    }

    (dest.to_string(), comp.to_string(), jump.to_string())
}




pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbolecless() {
        let contents = "\
@16
M=1
@17
M=0
@16
D=M
@0
D=D-M
@18
D;JGT
@16
D=M
@17
M=D+M
@16
M=M+1
@4
0;JMP
@17
D=M";

        let expected = vec![
            "0000000000010000",
            "1110111111001000",
            "0000000000010001",
            "1110101010001000",
            "0000000000010000",
            "1111110000010000",
            "0000000000000000",
            "1111010011010000",
            "0000000000010010",
            "1110001100000001",
            "0000000000010000",
            "1111110000010000",
            "0000000000010001",
            "1111000010001000",
            "0000000000010000",
            "1111110111001000",
            "0000000000000100",
            "1110101010000111",
            "0000000000010001",
            "1111110000010000",
        ];

        let actual = process(&contents);

        assert_eq!(expected, actual);
    }
}
