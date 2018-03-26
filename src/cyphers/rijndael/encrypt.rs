use fields::rijndael::invert;
use fields::rijndael::mul;

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
}

pub fn byte_substitution(buf: &[u8]) -> Vec<u8> {
    buf.iter().map(|&b| SBOX[b as usize]).collect()
}

pub fn shift_rows(buf: &[u8]) -> Vec<u8> {
    let mut block = Vec::new();
    for i in 0..4 {
        let row = [
            buf[(5*i) % 16],
            buf[(5*i + 4) % 16],
            buf[(5*i + 8) % 16],
            buf[(5*i + 12) % 16]
        ];
        block.extend_from_slice(&row);
    }
    block
}

pub fn mix_columns(buf: &[u8]) -> Vec<u8> {
    let mut block = Vec::new();
    for i in 0..4 {
        let col = [
            mul(2, buf[4*i]) ^ buf[3 + 4*i] ^ buf[2 + 4*i] ^ mul(3, buf[1 + 4*i]),
            mul(2, buf[1 + 4*i]) ^ buf[4*i] ^ buf[3 + 4*i] ^ mul(3, buf[2 + 4*i]),
            mul(2, buf[2 + 4*i]) ^ buf[1 + 4*i] ^ buf[4*i] ^ mul(3, buf[3 + 4*i]),
            mul(2, buf[3 + 4*i]) ^ buf[2 + 4*i] ^ buf[1 + 4*i] ^ mul(3, buf[4*i]),
        ];
        block.extend_from_slice(&col);
    }
    block
}
