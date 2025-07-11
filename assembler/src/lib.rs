use std::error::Error;
use std::fs;
use std::io::{ BufWriter, Write };
use std::collections::HashMap;

pub mod processing;

fn get_dest_table() -> HashMap<&'static str, &'static str> {
    let mut dest = HashMap::new();
    dest.insert("", "000");
    dest.insert("M", "001");
    dest.insert("D", "010");
    dest.insert("MD", "011");
    dest.insert("A", "100");
    dest.insert("AM", "101");
    dest.insert("AD", "110");
    dest.insert("AMD", "111");
    dest
}

fn get_comp_table() -> HashMap<&'static str, &'static str> {
    let mut comp = HashMap::new();
    comp.insert("0", "0101010");
    comp.insert("1", "0111111");
    comp.insert("-1", "0111010");
    comp.insert("D", "0001100");
    comp.insert("A", "0110000");
    comp.insert("!D", "0001101");
    comp.insert("!A", "0110001");
    comp.insert("-D", "0001111");
    comp.insert("-A", "0110011");
    comp.insert("D+1", "0011111");
    comp.insert("A+1", "0110111");
    comp.insert("D-1", "0001110");
    comp.insert("A-1", "0110010");
    comp.insert("D+A", "0000010");
    comp.insert("D-A", "0010011");
    comp.insert("A-D", "0000111");
    comp.insert("D&A", "0000000");
    comp.insert("D|A", "0010101");
    comp.insert("M", "1110000");
    comp.insert("!M", "1110001");
    comp.insert("-M", "1110011");
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
    jump.insert("", "000");
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
    let output_file = fs::File::create("output.hack")?;
    let mut writer = BufWriter::new(output_file);

    let contents = fs::read_to_string(config.file_path)?;

    let cleaned_content = cleaning::clean(&contents);
    let results = processing::process(cleaned_content);

    for line in results {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}
/*______________________module clean___________________ */

mod cleaning {
    use regex::Regex;

    pub fn clean(input: &str) -> String {
        let block_comments = Regex::new(r"(?s)/\*.*?\*/").unwrap();
        let no_block = block_comments.replace_all(input, "");

        let line_comments = Regex::new(r"//.*").unwrap();
        let no_line = line_comments.replace_all(&no_block, "");

        no_line
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/*_____________________________________________________ */

/*_________________test module_________________________________ */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let contents =
            "\
        // This is a comment
        @2
        D=A // Set D to 2

        /* multi-line
        comment here */

        @3
        D=D+A

        @0
        M
        (LOOP)
        @LOOP";

        let expected = vec![
            "0000000000000010",
            "1110110000010000",
            "0000000000000011",
            "1110000010010000",
            "0000000000000000",
            "1111110000000000",
            "0000000000000110"
        ];

        let actual = processing::process(cleaning::clean(contents));

        assert_eq!(expected, actual);
    }

    fn advance_test() {
        let contents =
            "\
            // Declaration
// R0 --> end 
@SCREEN
D=A
@8192
D=D+A
@R0
M=D

/////////////////////////////
// listen for key press
(KEY)
@KBD
D=M
@BLACK
D;JNE
@KEY
0;JMP

// blacken the screen
(BLACK)

// n --> screen
@SCREEN
D=A
@n
M=D

(BLOOP)
// listen for key unpressed
@KBD
D=M
@WHITE
D;JEQ
// if (n == R0) go to end 
@R0
D=M
@n
D=D-M
@KEY
D;JEQ

// n ++
@n
A=M
M=-1
@n
M=M+1
@BLOOP
0;JMP

(END)
@KEY
0;JMP
// end 

(WHITE)
// first make the screen white
// n --> screen
@SCREEN
D=A
@n
M=D

// if (n == R0) go to end 
(WLOOP)
@R0
D=M
@n
D=D-M
@KEY
D;JEQ
// colore white 
@n
A=M
M=0
// n ++
@n
M=M+1
@WLOOP
0;JMP
// end whitening the screen

        ";

        let expected = vec![
"0100000000000000",
"1110110000010000",
"0010000000000000",
"1110000010010000",
"0000000000000000",
"1110001100001000",
"0110000000000000",
"1111110000010000",
"0000000000001100",
"1110001100000101",
"0000000000000110",
"1110101010000111",
"0100000000000000",
"1110110000010000",
"0000000000010000",
"1110001100001000",
"0110000000000000",
"1111110000010000",
"0000000000100011",
"1110001100000010",
"0000000000000000",
"1111110000010000",
"0000000000010000",
"1111010011010000",
"0000000000000110",
"1110001100000010",
"0000000000010000",
"1111110000100000",
"1110111010001000",
"0000000000010000",
"1111110111001000",
"0000000000010000",
"1110101010000111",
"0000000000000110",
"1110101010000111",
"0100000000000000",
"1110110000010000",
"0000000000010000",
"1110001100001000",
"0000000000000000",
"1111110000010000",
"0000000000010000",
"1111010011010000",
"0000000000000110",
"1110001100000010",
"0000000000010000",
"1111110000100000",
"1110101010001000",
"0000000000010000",
"1111110111001000",
"0000000000100111",
"1110101010000111",
        ];

        let actual = processing::process(cleaning::clean(contents));

        assert_eq!(expected, actual);
    }
}
