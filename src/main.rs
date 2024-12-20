use std::env;
use std::fs;
use std::fmt::Error;
mod arg_handler;
fn main() {
    let args: Vec<String> = env::args().collect();
    let Config: arg_handler::ConfigStruct = arg_handler::parse_args(args).unwrap();

    let contents = fs::read_to_string(Config.file_path)
        .expect("Failed to read file");

    if let Some(shift) = Config.caesar_shift {
        println!("Encrypted: {}", caesar(&contents, shift));
    }
    if let Some(key) = Config.vignere_key {
        println!("Encrypted: {}", vignere(&contents, key));
    }
}

//caesar: Takes an &String plaintext and u8 key 
//returns the result of a map() which applies a closure to shift 
//each alphabetic char in plaintext by the value of key
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

//Vignere: Takes an &String plaintext and u8 key
fn vignere(plaintext: &String, key: String) -> String {

    
    let key_shifts: Vec<u8> = key
    .chars()
    .filter(|c| c.is_ascii_alphanumeric())
    .map(|c| {
        let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
        c as u8 - base
    }).collect(); //Identify the offset of each character in the key String

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
    }).collect() //Apply polyalphabetic shifts on plaintext and return the result.
}

