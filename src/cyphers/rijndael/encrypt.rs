use fields::rijndael::invert;
use fields::rijndael::mul;
use cyphers::xor::fixed_xor;

lazy_static! {
    static ref SBOX: Vec<u8> = {
        let mut sbox: Vec<u8> = vec![0x63];
        sbox.extend((1u8..=255).map(|a| {
            let mut s = invert(a).unwrap();
            let mut r = 0;
            for _ in 0..5 {
                r = r ^ s;
                s = s.rotate_left(1);
            }
            r ^ 99
        }));
        sbox
    };
    static ref MUL2: Vec<u8> = (0u8..=255).map(|a| mul(a, 2)).collect();
    static ref MUL3: Vec<u8> = (0u8..=255).map(|a| mul(a, 3)).collect();
}

pub fn byte_substitution(buf: &[u8]) -> Vec<u8> {
    buf.iter().map(|&b| SBOX[b as usize]).collect()
}

pub fn shift_rows(buf: &[u8]) -> Vec<u8> {
    let mut block = Vec::new();
    for i in 0..4 {
        let row = [
            buf[(4*i) % 16],
            buf[(4*i + 5) % 16],
            buf[(4*i + 10) % 16],
            buf[(4*i + 15) % 16]
        ];
        block.extend_from_slice(&row);
    }
    block
}

pub fn mix_columns(buf: &[u8]) -> Vec<u8> {
    let mut block = Vec::new();
    for i in 0..4 {
        let col = [
            MUL2[buf[4*i] as usize] ^ buf[3 + 4*i] ^ buf[2 + 4*i] ^ MUL3[buf[1 + 4*i] as usize],
            MUL2[buf[1 + 4*i] as usize] ^ buf[4*i] ^ buf[3 + 4*i] ^ MUL3[buf[2 + 4*i] as usize],
            MUL2[buf[2 + 4*i] as usize] ^ buf[1 + 4*i] ^ buf[4*i] ^ MUL3[buf[3 + 4*i] as usize],
            MUL2[buf[3 + 4*i] as usize] ^ buf[2 + 4*i] ^ buf[1 + 4*i] ^ MUL3[buf[4*i] as usize],
        ];
        block.extend_from_slice(&col);
    }
    block
}

fn g(buf: &[u8], rc: u8) -> Vec<u8> {
    let mut vec: Vec<u8> = buf.iter()
        .map(|&b| SBOX[b as usize]).collect();
    vec.rotate_left(1);
    vec[0] ^= rc;
    vec
}

pub fn key_schedule(key: &[u8]) -> Vec<Vec<u8>> {
    let mut rc = 1;
    let mut last = key.to_vec();
    let mut round_keys = Vec::new();
    round_keys.push(last.clone());
    for _ in 1..11 {
        let mut next = Vec::new();

        let a = fixed_xor(&last[0..4], &g(&last[12..16], rc));
        let b = fixed_xor(&a, &last[4..8]);
        let c = fixed_xor(&b, &last[8..12]);
        let d = fixed_xor(&c, &last[12..16]);

        next.extend_from_slice(&a);
        next.extend_from_slice(&b);
        next.extend_from_slice(&c);
        next.extend_from_slice(&d);

        last = next.clone();
        round_keys.push(next);
        rc = mul(rc, 2);
    }
    round_keys
}

