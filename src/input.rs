use std::collections::HashMap;
use std::env;
use std::fs;

pub fn load_input() -> HashMap<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file.toml>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    let data: HashMap<String, String> =
        toml::from_str(&contents).expect("Failed to parse TOML from input file");

    data
}
