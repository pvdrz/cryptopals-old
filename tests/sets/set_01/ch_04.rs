use cryptopals::utils::load_as_bytes;
use cryptopals::decrypt::break_single_xor;
use cryptopals::xor::single_xor;
use cryptopals::encoding::base16::Base16;

#[test]
fn challenge_04() {
    let buf = load_as_bytes("./data/4.txt").unwrap();
    let (chunk, _, _): (Vec<u8>, u8, f64) = buf.split(|&x| x == b'\n').filter_map(|chunk| {
        let b16 = Base16::new();
        let chunk = b16.decode(chunk);
        let (key, chisq) = break_single_xor(&chunk)?;
        if chisq.is_nan() {
            None
        } else {
            Some((single_xor(&chunk, key), key, chisq))
        }
    }).min_by(|x, y| x.2.partial_cmp(&y.2).unwrap()).unwrap();
    assert_eq!(chunk, b"Now that the party is jumping\n");
}
