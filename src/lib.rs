#![crate_type = "lib"]
#![crate_name = "suruga"]

#[macro_use]
extern crate log;
extern crate rand;
extern crate num;
extern crate crypto;

#[macro_use]
extern crate enum_primitive;

pub use client::TlsClient;

#[macro_use]
pub mod macros;
pub mod util;

pub mod tls_result;
#[macro_use]
pub mod tls_item;

// TLS AEAD cipehrsuites
pub mod crypto2;
pub mod cipher;

pub mod signature;
pub mod alert;
pub mod handshake;

pub mod tls;
pub mod client;

#[cfg(test)] mod test;
