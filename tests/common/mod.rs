use std::env;

use assert2::let_assert;

pub fn ensure_api_key() -> String {
    let_assert!(Ok(key) = env::var("WALLHAVENRS_TESTKEY"));
    key
}
