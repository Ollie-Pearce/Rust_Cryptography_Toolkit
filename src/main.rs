use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let contents = match fs::read_to_string(path){
        Ok(result) => result,
        Err(e) => panic!("Error: {e}"),
    };
    //println!("contents: {contents}");
    let x = ceasar(contents);

    println!("Encrypted: {x}");

}

fn ceasar(plaintext: String ) -> String {
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