pub fn hamming(buf_a: &[u8], buf_b: &[u8]) -> u32 {
    buf_a.iter().zip(buf_b.iter()).map(|(a, b)| (a ^ b).count_ones()).sum()
}
