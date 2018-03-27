use cryptopals::cyphers::rijndael::Cypher;
use cryptopals::encoding::base16::Base16;

#[test]
fn encrypt_0() {
    let b16 = Base16::new();

    let key =        b16.decode(b"00000000000000000000000000000000");
    let plaintext =  b16.decode(b"f34481ec3cc627bacd5dc3fb08f273e6");
    let cyphertext = b16.decode(b"0336763e966d92595a567cc9ce537f5e");

    let cypher = Cypher::new(key);

    assert_eq!(cyphertext, cypher.encrypt(&plaintext));
}

#[test]
fn encrypt_1() {
    let b16 = Base16::new();

    let key =        b16.decode(b"00000000000000000000000000000000");
    let plaintext =  b16.decode(b"9798c4640bad75c7c3227db910174e72");
    let cyphertext = b16.decode(b"a9a1631bf4996954ebc093957b234589");

    let cypher = Cypher::new(key);

    assert_eq!(cyphertext, cypher.encrypt(&plaintext));
}

#[test]
fn encrypt_2() {
    let b16 = Base16::new();

    let key =        b16.decode(b"00000000000000000000000000000000");
    let plaintext =  b16.decode(b"96ab5c2ff612d9dfaae8c31f30c42168");
    let cyphertext = b16.decode(b"ff4f8391a6a40ca5b25d23bedd44a597");

    let cypher = Cypher::new(key);

    assert_eq!(cyphertext, cypher.encrypt(&plaintext));
}

#[test]
fn encrypt_3() {
    let b16 = Base16::new();

    let key =        b16.decode(b"00000000000000000000000000000000");
    let plaintext =  b16.decode(b"6a118a874519e64e9963798a503f1d35");
    let cyphertext = b16.decode(b"dc43be40be0e53712f7e2bf5ca707209");

    let cypher = Cypher::new(key);

    assert_eq!(cyphertext, cypher.encrypt(&plaintext));
}

#[test]
fn encrypt_4() {
    let b16 = Base16::new();

    let key =        b16.decode(b"00000000000000000000000000000000");
    let plaintext =  b16.decode(b"cb9fceec81286ca3e989bd979b0cb284");
    let cyphertext = b16.decode(b"92beedab1895a94faa69b632e5cc47ce");

    let cypher = Cypher::new(key);

    assert_eq!(cyphertext, cypher.encrypt(&plaintext));
}

#[test]
fn encrypt_5() {
    let b16 = Base16::new();

    let key =        b16.decode(b"00000000000000000000000000000000");
    let plaintext =  b16.decode(b"b26aeb1874e47ca8358ff22378f09144");
    let cyphertext = b16.decode(b"459264f4798f6a78bacb89c15ed3d601");

    let cypher = Cypher::new(key);

    assert_eq!(cyphertext, cypher.encrypt(&plaintext));
}

#[test]
fn encrypt_6() {
    let b16 = Base16::new();

    let key =        b16.decode(b"00000000000000000000000000000000");
    let plaintext =  b16.decode(b"58c8e00b2631686d54eab84b91f0aca1");
    let cyphertext = b16.decode(b"08a4e2efec8a8e3312ca7460b9040bbf");

    let cypher = Cypher::new(key);

    assert_eq!(cyphertext, cypher.encrypt(&plaintext));
}
