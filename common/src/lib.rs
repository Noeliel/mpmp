use crate::error::Error;

pub mod config;
pub mod error;
pub mod socket;

#[cfg(not(any(target_os = "windows", target_vendor = "apple")))]
#[path = "crypto_impl/openssl.rs"]
mod crypto_impl;

#[cfg(target_os = "windows")]
#[path = "crypto_impl/native_tls.rs"]
mod crypto_impl;

pub type Result<T> = std::result::Result<T, Error>;
