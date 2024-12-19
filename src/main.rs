use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let contents = match fs::read_to_string(path){
        Ok(result) => result,
        Err(e) => panic!("Error: {e}"),
    };
    println!("contents: {contents}");
    ceasar(contents);

}

fn ceasar(plaintext: String ) {
    for c in plaintext.chars().enumerate() {
        let x = (c.1 as u8);
    }
}