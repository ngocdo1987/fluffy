use jsonwebtoken::{TokenData, Validation, Header};
use serde_derive::{Deserialize, Serialize};
//use serde::de::DeserializeWoned;
use serde::{de, ser};
use crate::datetime;

/// secret key
const KEY: [u8; 16] = *include_bytes!("./secret.key");

/// 1 day
pub const DAY_ONE: u64 = 86400;

/// User token
#[derive(Serialize, Deserialize)]
pub struct UserToken { 
    pub exp: u64, //Expiration
    pub id: usize, //user ID
    pub name: String, //user name
    //pub token: String, //token
}

/// Generate token
pub fn encode(id: usize, name: &String) -> String { 
    let now = datetime::timestamp();
    //let token = random::rand_str(32);
    let playload = UserToken { 
        exp: now + DAY_ONE, 
        id,
        name: name.to_owned(),  //token,
    };
    jsonwebtoken::encode(&Header::default(), &playload, &KEY).unwrap()
}

    /// Generate token -custom format
    pub fn encode_by<T: ser::Serialize>(data: &T) -> String { 
    //let token = random::rand_str(32);
    jsonwebtoken::encode::<T>(&Header::default(), &data, &KEY).unwrap()
}

/// Decoding Authorization field -default method
pub fn decode(token: &str) 
    -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(token, &KEY, &Validation::default())
}

/// Decoding Authorization field -custom method
pub fn decode_by<T: de::DeserializeOwned>(token: &str) 
    -> jsonwebtoken::errors::Result<TokenData<T>> {
    jsonwebtoken::decode::<T>(token, &KEY, &Validation::default())
}

/// Whether the verification is correct
pub fn verify<T: ser::Serialize>(_token_data: &TokenData<T>) -> Result<String, String> { 
    Ok("success".to_owned())
}
