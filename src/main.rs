use std::env;
use std::fs;
use std::fmt::Error;

struct Config {
    caesar: (bool, i32),
    file_path:  String,
}
fn main() {

    let mode: Config = parse_args(env::args().collect());

    let contents = match fs::read_to_string(mode.file_path){
        Ok(result) => result,
        Err(e) => panic!("Error: {e}"),
    };
    if mode.caesar.0 == true { let x = caesar(contents); 
        println!("Encrypted: {x}");
    }
    

    

}

fn caesar(plaintext: String ) -> String {
    let mut ciphertext: String = Default::default();
    for c in plaintext.chars().enumerate() {
        if c.1.is_ascii_alphabetic() {
            let x = (c.1 as u8);
            let y = x + 1;
            ciphertext.insert(c.0, y as char);
        } else {
            ciphertext.insert(c.0, c.1);
        }
    }
    ciphertext
}

fn parse_args(args: Vec<String>) -> Config {
    for (index, arg) in args.iter().enumerate() {
        if arg == "-caesar" {

            if (args[index+1].parse::<i32>().is_ok()){
                return Config{ caesar: (true, args[index+1].parse::<i32>().unwrap()), file_path: args[1].clone()}
            } else {
                panic!("Invalid arguments");
            }
        }
    }
    Config{ caesar: (false, 0),file_path: args[1].clone()}
}
