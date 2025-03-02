pub fn left_rotate(x: u32, c: u32) -> u32 {
    (x << c) | (x >> (32 - c))
}