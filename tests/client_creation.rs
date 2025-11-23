//! Test the 2 client creation functions, `new` and `with_key`

use assert2::assert;
use wallhaven_rs::WallhavenClient;

#[test]
fn test_new() {
    assert!(let Ok(_) = WallhavenClient::new());
}

#[test]
fn test_with_key() {
    assert!(let Ok(_) = WallhavenClient::with_key("ANYSTR"));
}
