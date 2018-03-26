use cryptopals::fields::rijndael::mul;

#[test]
fn mul_0() {
    assert_eq!(0, mul(0, 0));
}

#[test]
fn mul_1() {
    assert_eq!(1, mul(0x53, 0xca));
}

#[test]
fn mul_2() {
    assert_eq!(1, mul(0xc2, 0x2f));
}
