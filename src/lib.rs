#![allow(clippy::multiple_crate_versions)]

//! # wallhaven-rs
//!
//! `wallhaven-rs` is a wallhaven api wrapper.
//!
//! The entry point of everything is [`WallhavenClient`].

mod client;
mod models;

pub use client::*;
/// We export as a dependant may need Mime for type matching and not want to include it himself
pub use mime;
pub use models::*;
