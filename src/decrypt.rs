
fn frequency_count(buf: &[u8]) -> Vec<(u8, usize)> {
    let mut vec = buf.to_vec();
    vec.sort();
    vec.dedup();
    vec.into_iter().map(|b| {
        (b, buf.iter().filter(|&c| *c == b).count())
    }).collect()
}

pub fn break_single_xor(buf: &[u8], guess: u8) -> Option<u8> {
    frequency_count(buf)
        .iter()
        .max_by_key(|&&(_, c)| c)
        .map(|&(b, _)| b ^ guess)
}
