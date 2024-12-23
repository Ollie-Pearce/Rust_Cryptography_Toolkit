use std::env;
use std::fs;
use std::fmt::Error;
use std::process::abort;
mod arg_handler;
use rand::Rng;
use reikna::factor::coprime;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match arg_handler::parse_args(args){
        Ok(config) => config,
        Err(e) => return print_help(),
    };



    let contents = fs::read_to_string(config.file_path)
        .expect("Failed to read file");

    if let Some(shift) = config.caesar_shift {
        println!("Encrypted: {}", caesar(&contents, shift));
    }
    if let Some(key) = config.vignere_key {
        println!("Encrypted: {}", vignere(&contents, key));
    }
    if let Some(key) = config.rsa_key {
        println!("Encrypted: {}", rsa(&contents, key));
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


fn rsa(plaintext: &String, key: (u64, u64) ) -> String {
    let pub_key = key.0 * key.1;
    let pub_key_phi = (key.0-1)*(key.1-1);
    let mut rng = rand::thread_rng();

    let e = loop {
        let candidate = rng.gen_range(1..pub_key_phi);
        if coprime(candidate, pub_key) {
            break candidate;
        }
    };
    //choose e: 1 < e < phi(pub_key) && coprime (pub_key)
    //e and phi(pub_key) are the lock

    let d = (1..).find_map(|i| {
        let candidate = e * i;
        if candidate % pub_key_phi == 1 {
            return Some(candidate)
        } else {
            return None
        }
    }).unwrap();
    //Choose d such that  d*e % phi(pub_key) = 1

    //encrypt = phi(pub_key), e
    //decrypt = phi(pub_key), d

    println!("Ecryption keys: {pub_key_phi} {e}");
    println!("Decryption keys: {pub_key_phi} {d}");

    return "Hello World".to_string();
}


fn print_help() {
    println!("cargo run [FILEPATH] [CIPHER] [KEY]");
    println!("Ciphers: ");
    println!("\t -ceaesar [SHIFT]");
    println!("\t -vignere [KEY]");
    println!("\t -rsa [KEY 1] [KEY 2]");
    ()
}