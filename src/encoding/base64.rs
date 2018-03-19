use std::collections::HashMap;

pub struct Base64 {
    enc: Vec<u8>,
    dec: HashMap<u8, u8>,
    padding: u8
}

impl Base64 {
    pub fn new() -> Self {
        let enc: Vec<u8> = (b'A'..b'Z'+1)
            .chain(b'a'..b'z'+1)
            .chain(b'0'..b'9'+1)
            .chain([b'+', b'/'].iter().cloned())
            .collect();

        let dec = enc.iter().cloned().zip(0u8..64).collect();

        Base64 {
            enc: enc,
            dec: dec,
            padding: b'='
        }
    }

    pub fn encode(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut iter = bytes.iter();
        let (mut c1, mut c2, mut c3);
        while {
            c1 = iter.next().map(|&x| x as usize);
            c2 = iter.next().map(|&x| x as usize);
            c3 = iter.next().map(|&x| x as usize);
            c1.is_some()
        } {
            // 0b11111100 = 252
            // 0b00000011 = 3
            // 6000 = 240
            // 311 = 15
            // 4800 = 192
            // 1511 = 63
            let b1 = c1.unwrap();
            result.push(self.enc[(b1 & 252) >> 2]);
            if let Some(b2) = c2 {
                result.push(self.enc[((b1 & 3) << 4) + ((b2 & 240) >> 4)]);
                if let Some(b3) = c3 {
                    result.push(self.enc[((b2 & 15) << 2) + ((b3 & 192) >> 6)]);
                    result.push(self.enc[(b3 & 63)]); 
                } else {
                    result.push(self.enc[((b2 & 15) << 2)]);
                    result.push(self.padding);
                }
            } else {
                result.push(self.enc[((b1 & 3) << 4)]);
                result.push(self.padding);
                result.push(self.padding);
            }
        } 
        result
    }

    pub fn decode(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut iter = bytes.iter();
        let (mut c1, mut c2, mut c3, mut c4);
        while {
            c1 = iter.next().and_then(|x| self.dec.get(x));
            c2 = iter.next().and_then(|x| self.dec.get(x));
            c3 = iter.next().and_then(|x| self.dec.get(x));
            c4 = iter.next().and_then(|x| self.dec.get(x));
            c1.is_some()
        } {
            // 0b110000 = 48
            // 0b001111 = 15
            // 0b111100 = 60
            // 0b000011 = 3
            let b1 = c1.unwrap();
            let b2 = c2.unwrap();
            result.push((b1 << 2) + ((b2 & 48) >> 4));
            if let Some(b3) = c3 {
                result.push(((b2 & 15) << 4) + ((b3 & 60) >> 2));
                if let Some(b4) = c4 {
                    result.push(((b3 & 3) << 6) + b4);
                }
            }
        }      
        result
    }
}
