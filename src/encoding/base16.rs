use std::collections::HashMap;

pub struct Base16 {
    enc: Vec<u8>,
    dec: HashMap<u8, u8>
}

impl Base16 {
    pub fn new() -> Self {
        let enc: Vec<u8> = (b'0'..b'9'+1)
            .chain(b'a'..b'f'+1)
            .collect();

        let dec = enc.iter().cloned().zip(0u8..16).collect();

        Base16 {
            enc: enc,
            dec: dec
        }
    }

    pub fn encode(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        for c in bytes {
            result.push(self.enc[((c & 0b11110000) >> 4) as usize]);
            result.push(self.enc[(c & 0b00001111) as usize]);
        } 
        result
    }

    pub fn decode(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut iter = bytes.iter();
        let (mut c1, mut c2);
        while {
            c1 = iter.next().and_then(|x| self.dec.get(x));
            c2 = iter.next().and_then(|x| self.dec.get(x));
            c1.is_some()
        } {
            let b1 = c1.unwrap();
            let b2 = c2.unwrap();
            result.push((b1 << 4) + b2);
        }
        result
    }
}
