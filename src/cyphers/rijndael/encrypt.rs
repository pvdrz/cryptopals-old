use fields::rijndael::invert;

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
