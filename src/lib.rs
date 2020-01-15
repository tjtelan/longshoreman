//! # Longshoreman
//!
//! Asynchronous Docker client in pure rust.

#![deny(clippy::all, missing_docs, missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![allow(dead_code)]

mod compat;
mod docker;
mod error;
mod http_client;

use compat::Compat;
pub use docker::{images, Docker};
pub use error::{Error, Result};
