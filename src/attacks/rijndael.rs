pub fn detect_ecb(buffer: &[u8]) -> usize {
    buffer.chunks(16)
        .map(|c_a| buffer.chunks(16).filter(|&c_b| c_a == c_b).count())
        .sum()
}
