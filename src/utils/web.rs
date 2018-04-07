use std::collections::HashMap;

pub fn kv_parse(text: &str) -> HashMap<String, String> {
    let mut dict = HashMap::new();
    for item in text.split("&") {
        let item: &[&str] = &item.split("=").collect::<Vec<_>>();
        if let [key, value] = item {
            dict.insert(key.to_string(), value.to_string());
        }
    }    
    dict
}

pub fn profile_for(email: &str) -> String {
    let mut email = email.to_string();
    email.retain(|c| (c != '=') && (c != '&'));
    format!("email={}&uid=10&role=user", email)
}
