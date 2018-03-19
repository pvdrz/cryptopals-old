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
        let mut region: u8 = 0;
        let mut tail: usize = 0;
        for &b in bytes {
            let c = b as usize;
            // 0b11111100 = 252
            // 0b00000011 = 3
            // 6000 = 240
            // 311 = 15
            // 4800 = 192
            // 1511 = 63 
            if region == 0 {
                result.push(self.enc[(c & 252) >> 2]);
                tail = (c & 3) << 4;
            } else if region == 1 {
                result.push(self.enc[tail + ((c & 240) >> 4)]);
                tail = (c & 15) << 2;
            } else {
                result.push(self.enc[tail + ((c & 192) >> 6)]);
                result.push(self.enc[c & 63]);
            }
            region = (region + 1) % 3;
        }
        if region != 0 {
            result.push(self.enc[tail]);
            result.extend((0..3 - region).map(|_| self.padding));
        }
        result
    }

    pub fn decode(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut region: u8 = 0;
        let mut tail: u8 = 0;
        for b in bytes {
            if let Some(c) = self.dec.get(b) {
                if region == 0 {
                    tail = c << 2;
                } else if region == 1 {
                    result.push(tail + ((c & 48) >> 4)); 
                    tail = (c & 15) << 4; 
                } else if region == 2 {
                    result.push(tail + ((c & 60) >> 2));
                    tail = (c & 3) << 6; 
                } else {
                    result.push(tail + c);
                }
                region = (region + 1) % 4;
            }
        }
        result
    }
}
