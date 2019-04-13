use lazy_static::lazy_static;

use crate::fields::rijndael::invert;
use crate::fields::rijndael::mul;

lazy_static! {
    static ref SBOX: Vec<u8> = {
        let mut sbox: Vec<u8> = vec![0; 256];
        sbox[0x63] = 0;
        for a in 1u8..=255 {
            let mut s = invert(a).unwrap();
            let mut r = 0;
            for _ in 0..5 {
                r = r ^ s;
                s = s.rotate_left(1);
            }
            sbox[(r ^ 99) as usize] = a;
        }
        sbox
    };
    static ref MUL9: Vec<u8> = (0u8..=255).map(|a| mul(a, 9)).collect();
    static ref MUL11: Vec<u8> = (0u8..=255).map(|a| mul(a, 11)).collect();
    static ref MUL13: Vec<u8> = (0u8..=255).map(|a| mul(a, 13)).collect();
    static ref MUL14: Vec<u8> = (0u8..=255).map(|a| mul(a, 14)).collect();
}

pub fn byte_substitution(buf: &[u8]) -> Vec<u8> {
    buf.iter().map(|&b| SBOX[b as usize]).collect()
}

pub fn shift_rows(buf: &[u8]) -> Vec<u8> {
    let mut block = Vec::new();
    for i in 0..4 {
        let row = [
            buf[(4*i) % 16],
            buf[(4*i + 13) % 16],
            buf[(4*i + 10) % 16],
            buf[(4*i + 7) % 16]
        ];
            block.extend_from_slice(&row);
    }
    block
}

pub fn mix_columns(buf: &[u8]) -> Vec<u8> {
    let mut block = Vec::new();
    for i in 0..4 {
        let a = buf[4*i] as usize;
        let b = buf[4*i + 1] as usize;
        let c = buf[4*i + 2] as usize;
        let d = buf[4*i + 3] as usize;
        let col = [
            MUL14[a] ^ MUL11[b] ^ MUL13[c] ^ MUL9[d],
            MUL9[a]  ^ MUL14[b] ^ MUL11[c] ^ MUL13[d],
            MUL13[a] ^ MUL9[b]  ^ MUL14[c] ^ MUL11[d],
            MUL11[a] ^ MUL13[b] ^ MUL9[c]  ^ MUL14[d]
        ];
            block.extend_from_slice(&col);
    }
    block
}
