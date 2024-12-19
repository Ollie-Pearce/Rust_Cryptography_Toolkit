use std::env;
use std::fs;
use std::fmt::Error;

struct Config {
    caesar: (bool, u8),
    file_path:  String,
}
fn main() {

    let mode: Config = parse_args(env::args().collect());

    let contents = match fs::read_to_string(mode.file_path){
        Ok(result) => result,
        Err(e) => panic!("Error: {e}"),
    };
    if mode.caesar.0 == true { let x = caesar(contents, mode.caesar.1); 
        println!("Encrypted: {x}");
    }
    

    

}

fn caesar(plaintext: String , shift: u8) -> String {
plaintext
    .chars()
    .map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = c as u8 - base;
            let shifted = (offset + shift) % 26;
            (base + shifted) as char
        } else {
            c
        }
    }).collect()
}

fn parse_args(args: Vec<String>) -> Config {
    for (index, arg) in args.iter().enumerate() {
        if arg == "-caesar" {

            if (args[index+1].parse::<u8>().is_ok()){
                return Config{ caesar: (true, args[index+1].parse::<u8>().unwrap()), file_path: args[1].clone()}
            } else {
                panic!("Invalid arguments");
            }
        }
    }
    Config{ caesar: (false, 0),file_path: args[1].clone()}
}
