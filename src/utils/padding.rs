pub fn pkcs7(buf: &[u8], blocksize: usize) -> Vec<u8> {
    let mut vec = buf.to_vec();
    let n = match buf.len() % blocksize {
        0 => 0,
        k => blocksize - k
    };
    vec.extend_from_slice(&vec![n as u8; n]);
    vec
}
