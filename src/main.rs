use std::env;
use std::fs;


struct Config {
    caesar: bool,
    file_path:  String,
}
fn main() {

    let mode: Config = parse_args(env::args().collect());

    let contents = match fs::read_to_string(mode.file_path){
        Ok(result) => result,
        Err(e) => panic!("Error: {e}"),
    };
    //println!("contents: {contents}");
    let x = caesar(contents);

    println!("Encrypted: {x}");

}

fn caesar(plaintext: String ) -> String {
    let mut ciphertext: String = Default::default();
    for c in plaintext.chars().enumerate() {
        if (c.1.is_ascii_alphabetic()) {
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
    for i in &args {
        if i == "-caesar" {
            return Config{ caesar: true, file_path: args[1].clone()}
        }
    }
    Config{ caesar: false, file_path: args[1].clone()}
}
