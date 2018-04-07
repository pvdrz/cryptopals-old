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
    if c_blocks <= 1 + 2*diff {
        None
    } else {
        let mut blocks = cyphertext.chunks(blocksize)
            .skip(diff)
            .take(c_blocks - 2 * diff);
        let first_block = blocks.next().unwrap(); // there is more than one
        Some(blocks.all(|b| b == first_block))
    }
}

pub fn find_blocksize(black_box: &Fn(&[u8]) -> Vec<u8>, limit: usize) -> Option<usize> {
    let mut plaintext = vec![b'a'];
    let mut cyphertext = black_box(&plaintext);
    let mut last_len = cyphertext.len();
    for _ in 0..limit {
        plaintext.push(b'a');
        cyphertext = black_box(&plaintext);
        let diff = cyphertext.len() - last_len;
        if diff > 0 {
            return Some(diff);
        } else {
            last_len = cyphertext.len();
        }
    }
    None
}

pub fn baat_ecb_decrypt(black_box: &Fn(&[u8]) -> Vec<u8>, blocksize: usize, n_blocks: usize) -> Vec<u8> {
    let mut decrypted = Vec::new();
    for n in 1..=n_blocks {
        let mask = (n - 1) * blocksize..n * blocksize;
        for i in 1..=blocksize {
            let mut plaintext = vec![b'a'; blocksize - i];
            let cyphertext = black_box(&plaintext)[mask.clone()].to_vec();
            plaintext.extend_from_slice(&decrypted);

            if let Some(byte) = (0u8..=255).find(|&b| {
                let mut input = plaintext.clone();
                input.push(b);
                cyphertext == black_box(&input)[mask.clone()].to_vec()
            }) {
                decrypted.push(byte);
            } else {
                break;
            }
        }
    }

    decrypted
}
