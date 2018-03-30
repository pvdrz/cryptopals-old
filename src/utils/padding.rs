pub fn pkcs7(buf: &[u8], blocksize: usize) -> Vec<u8> {
    let mut vec = buf.to_vec();
    let n = blocksize - (buf.len() % blocksize);
    vec.extend_from_slice(&vec![n as u8; n]);
    vec
}
