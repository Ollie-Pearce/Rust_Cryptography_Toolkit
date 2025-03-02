use super::left_rotate::left_rotate;

// MD5 transformation function that processes each block
pub fn md5_transform(blocks: &[Vec<u8>], mut a: u32, mut b: u32, mut c: u32, mut d: u32) -> (u32, u32, u32, u32) {
    println!("Starting MD5 transformation...");
    println!("Initial state variables: a = {:08x}, b = {:08x}, c = {:08x}, d = {:08x}", a, b, c, d);

    for (block_index, block) in blocks.iter().enumerate() {
        println!("\nProcessing block {}: {:?}", block_index + 1, block);

        // Step 1: Process each block (converting bytes to u32)
        let mut block_data = vec![0u32; 16];
        for i in 0..16 {
            block_data[i] = u32::from_le_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        println!("Block data (u32): {:?}", block_data);

        // Step 2: Initialise temporary variables for the current block
        let mut a_temp = a;
        let mut b_temp = b;
        let mut c_temp = c;
        let mut d_temp = d;

        println!("Temporary state variables: a_temp = {:08x}, b_temp = {:08x}, c_temp = {:08x}, d_temp = {:08x}", a_temp, b_temp, c_temp, d_temp);

        // Step 3: Over 64 rounds, mix the data and produce a hash
        for i in 0..64 {
            let (f, g) = if i < 16 {
                ((b_temp & c_temp) | ((!b_temp) & d_temp), i)
            } else if i < 32 {
                ((d_temp & b_temp) | ((!d_temp) & c_temp), (5 * i + 1) % 16)
            } else if i < 48 {
                (b_temp ^ c_temp ^ d_temp, (3 * i + 5) % 16)
            } else {
                (c_temp ^ (b_temp | (!d_temp)), (7 * i) % 16)
            };

            let temp = b_temp
                .wrapping_add(left_rotate(a_temp.wrapping_add(f).wrapping_add(block_data[g]).wrapping_add(0), 7));

            a_temp = d_temp;
            d_temp = c_temp;
            c_temp = b_temp;
            b_temp = temp;

            // Only print after round 64
            if i == 63 {
                println!("After round 64: a_temp = {:08x}, b_temp = {:08x}, c_temp = {:08x}, d_temp = {:08x}", a_temp, b_temp, c_temp, d_temp);
            }
        }

        // Step 4: Update the state variables
        a = a.wrapping_add(a_temp);
        b = b.wrapping_add(b_temp);
        c = c.wrapping_add(c_temp);
        d = d.wrapping_add(d_temp);

        println!("Updated state variables: a = {:08x}, b = {:08x}, c = {:08x}, d = {:08x}", a, b, c, d);
    }

    // Step 5: Return the final MD5 hash components
    println!("Final MD5 hash components: a = {:08x}, b = {:08x}, c = {:08x}, d = {:08x}", a, b, c, d);
    (a, b, c, d)
}