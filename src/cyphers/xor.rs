pub fn fixed_xor(buf_a: &[u8], buf_b: &[u8]) -> Vec<u8> {
    buf_a.iter().zip(buf_b.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn single_xor(buf: &[u8], key: u8) -> Vec<u8> {
    buf.iter().map(|b| key ^ b).collect()
}

pub fn repeating_xor(buf: &[u8], key: &[u8]) -> Vec<u8> {
    buf.iter().zip(key.iter().cycle()).map(|(a, b)| a ^ b).collect()
}
