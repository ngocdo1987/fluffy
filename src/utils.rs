use md5;

use crate::random;

/// Security key
const SECRET_KEYS: &str = "!s@w4$qS%^(_123-=0Xha9452sLW^%sfa9)\\";

/// md5
#[inline]
pub fn md5_str(content: &str) -> String { 
    let encrypt = md5::compute(content);
    format!("{:x}", encrypt)
}

/// Generate password
#[inline]
pub fn get_password(real_password: &str, secret: &str) -> String { 
    let origin = format!("{}-{}-{}", real_password, SECRET_KEYS, secret);
    md5_str(origin.as_str())
}

/// Get initialization password
pub fn print_init_password() -> String { 
    let origin_password = "qwe123";
    let secret = random::rand_str(32);
    let encrypt_password = get_password(origin_password, secret.as_str());
    format!("origin: {}\nsecret = {}\npassword = {}", origin_password, secret, encrypt_password)
}

/// url decoding
pub fn decode(url: String) -> String {
    let mut decoded = String::from("");
    let mut skip = 0;
    for i in 0..url.len() {
        if skip != 0 {
            skip -= 1;
            continue;
        }
        let c: char = url.chars().nth(i).unwrap();
        if c == '%' {
            let left = url.chars().nth(i + 1).unwrap();
            let right = url.chars().nth(i + 2).unwrap();
            let byte = u8::from_str_radix(&format!("{}{}", left, right), 16).unwrap();
            decoded += &(byte as char).to_string();
            skip = 2;
        } else {
            decoded += &c.to_string();
        }
    }
    decoded
}
