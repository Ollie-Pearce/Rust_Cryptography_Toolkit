use std::env;
use std::fs;
use std::fmt::Error;

struct ConfigStruct {
    caesar_shift: Option<u8>,
    vignere_key: Option<String>,
    file_path: String,
}

fn main() {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let Config: ConfigStruct   = parse_args(args).unwrap();

    let contents = match fs::read_to_string(Config.file_path){
        Ok(result) => result,
        Err(e) => panic!("Error: {e}"),
    };

    if let Some(shift) = Config.caesar_shift {
        let x = caesar(contents.clone(), shift);
        println!("Encrypted: {x}");
    }
    if let Some(key) = Config.vignere_key {
        let y = vignere(contents.clone(), key);
        println!("Encrypted: {y}");
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

fn vignere(plaintext: String, key: String) -> String {

    let key_shifts: Vec<u8> = key
    .chars()
    .filter(|c| c.is_ascii_alphanumeric())
    .map(|c| {
        let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
        c as u8 - base
    }).collect();

    let mut key_iter = key_shifts.iter().cycle();

    plaintext
    .chars()
    .map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = c as u8 - base;
            let shift = key_iter.next().unwrap();
            let shifted = (offset + shift) % 26;
            (base + shifted) as char

        } else {
            c
        }
    }).collect()
}

fn parse_args(args: Vec<String>) -> Result<ConfigStruct, String> {
    if args.len() < 2 {
        return Err("Not enough arguments. Usage: program [options] <file_path>".to_string());
    }

    let file_path = args.first().unwrap().clone();

    let mut caesar_shift: Option<u8> = None;
    let mut vignere_key: Option<String> = None;

    let mut i = 1;
    while i < args.len() - 1 {
        match args[i].as_str() {

            "-caesar" => {
                if i + 1 >= args.len() {
                    return Err("Missing shift value after -caesar".to_string());
                }
                let shift_str = &args[i + 1];
                caesar_shift = Some(
                    shift_str
                        .parse()
                        .map_err(|_| "Invalid shift value for Caesar cipher".to_string())?,
                );
                i += 2;
            }
            
            "-vignere" => {
                if i + 1 >= args.len() {
                    return Err("Missing key after -vignere".to_string());
                }
                vignere_key = Some(args[i + 1].clone());
                i += 2;
            }

            _ => {
                i += 1;
            }
        }
    }

    Ok(ConfigStruct {
        caesar_shift,
        vignere_key,
        file_path,
    })
}
