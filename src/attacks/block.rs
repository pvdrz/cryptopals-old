pub fn detect_ecb(buffer: &[u8]) -> usize {
    buffer.chunks(16)
        .map(|c_a| buffer.chunks(16).filter(|&c_b| c_a == c_b).count())
        .sum()
}

pub fn ecb_oracle(black_box: &Fn(&[u8]) -> Vec<u8>, blocksize: usize, n_blocks: usize) -> Option<bool> {
    let plaintext = vec![b'a'; blocksize * n_blocks];
    let cyphertext = black_box(&plaintext);

    let c_blocks = cyphertext.len() / blocksize;
    let diff = c_blocks - n_blocks;
    if c_blocks - 2*diff <= 1 {
        None
    } else {
        let mut blocks = cyphertext.chunks(blocksize)
            .skip(diff)
            .take(c_blocks - 2 * diff);
        let first_block = blocks.next().unwrap(); // there is more than one
        Some(blocks.all(|b| b == first_block))
    }
}
