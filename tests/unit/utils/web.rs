use cryptopals::utils::web::{kv_parse, profile_for};
use std::collections::HashMap;

#[test]
fn kv_parse_0 () {
    let input = "foo=bar&baz=qux&zap=zazzle";
    let mut result = HashMap::new();
    result.insert("foo".to_string(), "bar".to_string());
    result.insert("baz".to_string(), "qux".to_string());
    result.insert("zap".to_string(), "zazzle".to_string());
    
    let output = kv_parse(input);
    assert_eq!(result, output);
}

#[test]
fn profile_for_0 () {
    let input = "foo@bar.com";
    let result = "email=foo@bar.com&uid=10&role=user".to_string();
    
    let output = profile_for(input);
    assert_eq!(result, output)
}

#[test]
fn profile_for_1 () {
    let input = "foo@bar.com&role=admin";
    let result = "email=foo@bar.comroleadmin&uid=10&role=user".to_string();
    
    let output = profile_for(input);
    assert_eq!(result, output)
}
