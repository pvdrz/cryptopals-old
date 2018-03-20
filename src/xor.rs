pub fn fixed_xor(buf_a: &[u8], buf_b: &[u8]) -> Vec<u8> {
    buf_a.iter().zip(buf_b.iter()).map(|(a, b)| a ^ b).collect()
}
