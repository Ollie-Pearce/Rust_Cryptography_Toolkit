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

    let contents = fs::read_to_string(Config.file_path)
        .expect("Failed to read file");

    if let Some(shift) = Config.caesar_shift {
        println!("Encrypted: {}", caesar(&contents, shift));
    }
    if let Some(key) = Config.vignere_key {
        println!("Encrypted: {}", vignere(&contents, key));
    }
}

fn caesar(plaintext: &String , shift: u8) -> String {
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

fn vignere(plaintext: &String, key: String) -> String {

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

    let file_path = args[0].clone();

    let mut caesar_shift: Option<u8> = None;
    let mut vignere_key: Option<String> = None;

    let mut i = 1;
    let mut iter = args[1..].iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-caesar" => {
                let shift_str = iter
                    .next()
                    .ok_or_else(|| "Missing shift value after caesar")?;
                let shift = shift_str
                    .parse()
                    .map_err(|_| "Invalid shift value for caesar")?;

                
                caesar_shift = Some(shift);
            }
            
            "-vignere" => {
                let key = iter
                    .next()
                    .ok_or_else(|| "Missing key after vignere")?;
                vignere_key = Some(key.to_string());
            }

            _ => {
                {}
            }
        }
    }

    Ok(ConfigStruct {
        caesar_shift,
        vignere_key,
        file_path,
    })
}
