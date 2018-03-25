pub fn mul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 == 1 {
            p = p ^ a;
        }
        if a & 128 == 128 {
            a = (a << 1) ^ 0x1b;
        } else {
            a = a << 1;
        }
        b = b >> 1;
    }
    p
}

pub fn invert(a: u8) -> Option<u8> {
    let mut inv = None;
    if a != 0 {
        for b in 0..=255 {
            if mul(a, b) == 1 {
                inv = Some(b);
                break;
            }
        }
    }
    inv
}
