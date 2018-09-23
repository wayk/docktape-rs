#![deny(missing_docs)]
#![deny(warnings)]

//! # Docktape
//!
//! Docktape is a wrapper for the Docker API (https://docs.docker.com/develop/sdk/).
//!
//! This crate is currently using Hyper v0.11.27 (https://docs.rs/crate/hyper/0.11.27) and will be updated very soon.

extern crate futures;
extern crate hyper;
#[cfg(not(target_os = "windows"))]
extern crate hyperlocal;
extern crate tokio_core;
extern crate serde_json;
#[macro_use]
extern crate percent_encoding;
extern crate log;
extern crate url;

extern crate chrono;

mod container;
mod image;
mod network;
mod volume;
mod socket;
mod docker;
mod utils;

pub use futures::Stream;
pub use futures::Future;
pub use hyper::{Client, Result, Request};
pub use tokio_core::reactor::Core;
pub use hyper::Uri;
pub use std::io::{self, Write};
pub use serde_json::Value;
pub use docker::Docker;
pub use hyper::Method;

#[cfg(not(target_os = "windows"))]
pub use hyperlocal::UnixConnector;
pub use socket::Socket;

pub use chrono::DateTime;