use std::env;
use std::process;

use assembler::Config;



fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem prasing arguments: {err}");
        process::exit(1);
    }) ;

    
    if let Err(e) = assembler::run(config) {
        eprintln!("Applicationi error: {e}");
        process::exit(1);
    }
}



