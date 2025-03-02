pub fn pad_message(message: &Vec<u8>) -> Vec<u8> {
    let mut padded = message.clone();

    println!("Original message: {:?}", message);
    println!("Original message length (bytes): {}", message.len());
    println!("Original message length (bits): {}", message.len() * 8);

    // 1. Append a single 1 bit (0x80)
    padded.push(0x80);
    println!("After appending 0x80: {:?}", padded);

    // 2. Calculate the number of zeroes needed
    // The message length in bits must be congruent to 448 modulo 512
    let message_len_bits = (message.len() as u64) * 8;
    let padding_len = (448 - (message_len_bits + 8) % 512) % 512; // +8 for the 0x80 byte
    let zeroes_needed = (padding_len / 8) as usize;

    println!("Zeroes needed: {}", zeroes_needed);

    // Append the required zeroes in one go
    padded.extend(vec![0; zeroes_needed]);
    println!("After padding with zeroes: {:?}", padded);

    // 3. Append the original message length in bits (as a 64-bit little-endian integer)
    padded.extend_from_slice(&message_len_bits.to_le_bytes());
    println!("After appending message length: {:?}", padded);

    println!("Final padded message length (bytes): {}", padded.len());
    println!("Final padded message length (bits): {}", padded.len() * 8);

    padded
}