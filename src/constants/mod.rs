/// Used to generate a key
pub const KEY: [u8; 16] = *include_bytes!("../secret.key");

/// authorization The name
///
/// Generally format
///
/// Authorization: Bearer xxxxxxxxx
pub const AUTHORIZATION: &'static str = "Authorization";

/// Number of seconds: One week
pub const ONE_WEEK: u64 = 86400 * 7;

/// Number of seconds: Ippen
/// 
/// Generally used to set the expiration time
pub const ONE_DAY: u64 = 86400;
