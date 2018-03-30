use cryptopals::attacks::block::detect_ecb;
use cryptopals::utils::io::load_as_bytes;
use cryptopals::encoding::base16::Base16;

#[test]
fn challenge_08() {
    let b16 = Base16::new();
    let bufs: Vec<u8> = load_as_bytes("data/8.txt").unwrap();
    let (_, count) = bufs.split(|&x| x == b'\n')
        .map(|b| (b, detect_ecb(&b16.decode(b))))
        .max_by_key(|&(_, x)| x).unwrap();
    assert_eq!(22, count);
}
