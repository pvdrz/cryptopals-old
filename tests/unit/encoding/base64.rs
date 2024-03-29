use cryptopals::encoding::base64::Base64;

#[test]
fn base64_encode_no_padding() {
    let b64 = Base64::new();
    let input = b"any carnal pleasur";
    let result = b"YW55IGNhcm5hbCBwbGVhc3Vy";
    assert_eq!(b64.encode(input), result);
}

#[test]
fn base64_encode_simple_padding_0() {
    let b64 = Base64::new();
    let input = b"any carnal pleasure.";
    let result = b"YW55IGNhcm5hbCBwbGVhc3VyZS4=";
    assert_eq!(b64.encode(input), result);
}

#[test]
fn base64_encode_simple_padding_1() {
    let b64 = Base64::new();
    let input = b"any carnal pleasu";
    let result = b"YW55IGNhcm5hbCBwbGVhc3U=";
    assert_eq!(b64.encode(input), result);
}

#[test]
fn base64_encode_double_padding_0() {
    let b64 = Base64::new();
    let input = b"any carnal pleasure";
    let result = b"YW55IGNhcm5hbCBwbGVhc3VyZQ==";
    assert_eq!(b64.encode(input), result);
}

#[test]
fn base64_encode_double_padding_1() {
    let b64 = Base64::new();
    let input = b"any carnal pleas";
    let result = b"YW55IGNhcm5hbCBwbGVhcw==";
    assert_eq!(b64.encode(input), result);
}

#[test]
fn base64_decode_no_padding() {
    let b64 = Base64::new();
    let input = b"YW55IGNhcm5hbCBwbGVhc3Vy";
    let result = b"any carnal pleasur";
    assert_eq!(b64.decode(input), result);
}

#[test]
fn base64_decode_simple_padding_0() {
    let b64 = Base64::new();
    let input = b"YW55IGNhcm5hbCBwbGVhc3VyZS4=";
    let result = b"any carnal pleasure.";
    assert_eq!(b64.decode(input), result);
}

#[test]
fn base64_decode_simple_padding_1() {
    let b64 = Base64::new();
    let input = b"YW55IGNhcm5hbCBwbGVhc3U=";
    let result = b"any carnal pleasu";
    assert_eq!(b64.decode(input), result);
}


#[test]
fn base64_decode_double_padding_0() {
    let b64 = Base64::new();
    let input = b"YW55IGNhcm5hbCBwbGVhc3VyZQ==";
    let result = b"any carnal pleasure";
    assert_eq!(b64.decode(input), result);
}

#[test]
fn base64_decode_double_padding_1() {
    let b64 = Base64::new();
    let input = b"YW55IGNhcm5hbCBwbGVhcw==";
    let result = b"any carnal pleas";
    assert_eq!(b64.decode(input), result);
}
