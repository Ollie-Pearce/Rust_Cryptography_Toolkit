use std::env;
use std::fs;
use std::fmt::Error;
use std::process::abort;
use rand::Rng;
use reikna::factor::coprime;
use base64;
//use modinverse::modinverse;
mod arg_handler;
mod mod_pow;

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
//Example usage: cargo run -- src/test.txt -caesar 8
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
//Example usage: cargo run -- src/test.txt -vignere SECRET
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

//RSA: Takes a &String plaintext and a tuple (u64, u64) as the public key
//Example usage: cargo run -- src/test.txt -rsa 2 7
//This function generates a public key (N, e) and a private key (N, d), 
//then encrypts the plaintext using the public key and returns the ciphertext.
fn rsa(plaintext: &String, key: (u64, u64)) -> String {
    println!("Starting RSA encryption...");

    // Step 1: Calculating N = p * q (public key modulus)
    println!("Step 1: Calculating N = p * q...");
    let pub_key = key.0 * key.1;
    println!("Step 1: Calculated N = {}", pub_key);

    // Step 2: Calculating phi(N) = (p-1) * (q-1)
    println!("Step 2: Calculating phi(N) = (p-1) * (q-1)...");
    let pub_key_phi = (key.0 - 1) * (key.1 - 1);
    println!("Step 2: Calculated phi(N) = {}", pub_key_phi);

    // Step 3: Finding valid e such that 1 < e < phi(N) and e is coprime with N, phi(N)
    println!("Step 3: Finding valid e...");
    let mut rng = rand::thread_rng();
    let e = loop {
        let candidate = rng.gen_range(2..pub_key_phi);
        if coprime(candidate, pub_key) && coprime(candidate, pub_key_phi) {
            println!("Step 3: Found valid e = {}", candidate);
            break candidate;
        }
    };

    // Step 4: Finding valid d such that (d * e) % phi(N) = 1

    /* UNLIMITED SEARCH RANGE BRUTE FORCE METHOD
    println!("Step 4: Finding valid d...");
    let d = (1..).find_map(|i| {
        let candidate = e * i;
        if candidate % pub_key_phi == 1 { // Ensure d * e ≡ 1 mod phi(N)
            println!("Step 4: Found valid d = {}", candidate);
            return Some(candidate);
        } else {
            None
        }
    }).unwrap();
    */

    /* LAZY BUT QUICK MOD INVERSE METHOD (remember to uncomment import)
    println!("Step 4: Finding valid d...");
    println!("e = {}", e);
    println!("pub_key_phi = {}", pub_key_phi);
    let d = modinverse(e, pub_key_phi).unwrap();
    println!("Step 4: Found valid d = {}", d);
    */

    /* Another slow brute force method which works but isn't idiomatically Rust-like
    println!("Step 4: Finding valid d...");
    let mut d = 0;
    for candidate in 1..pub_key_phi {
        if (e * candidate) % pub_key_phi == 1 {
            d = candidate;
            println!("Step 4: Found valid d = {}", d);
            break;
        } 
    }
    */

    // Fix of the original and idiomatic solution to finding d
    println!("Step 4: Finding valid d...");
    let d = (1..pub_key_phi).find_map(|candidate| {
        if (e * candidate) % pub_key_phi == 1 { // Ensure e*d(mod phi(N) ≡ 1
            println!("Step 4: Found valid d = {}", candidate);
            Some(candidate)
        } else {
            None
        }
    }).unwrap();

    // Step 5: Raw encryption of the plaintext using the public key (e, N)
    println!("Step 5: Encrypting the plaintext...");
    let encrypted: Vec<u64> = plaintext
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                let c_ascii = c.to_ascii_lowercase() as u64 - b'a' as u64;
                // RSA encryption: ciphertext = (plaintext^e) % N
                // ```Some((c_ascii.pow(e as u32)) % pub_key_phi)``` causes:
                // thread 'main' panicked at /rustc/99768c80a1c094a5cfc3b25a04e7a99de7210eae/library/core/src/num/mod.rs:1121:5:
                // attempt to multiply with overflow. AKA we need to calculate in smaller steps which fit into a u64

                // Apparently modular exponentiation fixes this by reducing the result at each step:
                Some(mod_pow::mod_pow(c_ascii, e, pub_key_phi))
            } else {
                None
            }
        })
        .collect();

    // Step 6: Printing the generated public and private keys
    println!("Step 6: Printing the public and private keys...");
    println!("Public key: ({}, {})", e, pub_key);
    println!("Private key: ({}, {})", d, pub_key);

    // Step 7: Converting the encrypted vector back to a string and returning it
    /* A pythonic solution kek
    let mut encrypted_str = String::new();

    for (index, num) in encrypted.iter().enumerate() {
        if index > 0 {
            encrypted_str.push(',');
        }
        encrypted_str.push_str(&num.to_string());
    }
    */

    /* My attempt at an idiomatic solution */
    // Note: Vector `encrypted` is dynamically sized but Arrays have a fixed size.
    let encrypted_str = (1..encrypted.len())
        .map(|i| encrypted[i].to_string()) // convert every element of vector to string
        .collect::<Vec<String>>() // collect results into a Vector of Strings
        .join(""); // join vector is strings into one string


    let base64_encrypted_str = base64::encode(&encrypted_str).expect("Base64 encoding failed");
    println!("Encryption complete");
    base64_encrypted_str
}


// MD5: Bad things are going to happen if you use this in any meaningful application due to arbitrary hashing collisions
// Takes an &String plaintext and returns its MD5 hash as a String
fn md5(plaintext: &String) -> String {
    // TODO: Implement MD5 hashing algorithm
    // Step 1: Initialise state variables

    // Step 2: Pad the message to make its length a multiple of 512 bits

    // Step 3: Divide the message into 512-bit blocks

    // Step 4: Process each block using 64 rounds of transformations

    // Step 5: Update the state variables

    // Step 6: Concatenate the final state to get a 128-bit hash
    String::new()
}

fn print_help() {
    println!("cargo run [FILEPATH] [CIPHER] [KEY]");
    println!("Ciphers: ");
    println!("\t -caesar [SHIFT]");
    println!("\t -vignere [KEY]");
    println!("\t -rsa [KEY 1] [KEY 2]");
    println!("\t -md5");
    ()
}