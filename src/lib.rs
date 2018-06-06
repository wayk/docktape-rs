#![deny(missing_docs)]
#![deny(warnings)]

//! # Docktape
//!
//! Docktape is a wrapper for the Docker API (https://docs.docker.com/develop/sdk/).
//!
//! It currently works with Unix sockets but soon it will be available for TCP sockets.
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
#[macro_use]
extern crate log;

mod container;
mod image;
mod network;
mod volume;
#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(target_os = "windows")]
mod tcp;
mod docker;

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
pub use unix::{UnixSocket};
#[cfg(not(target_os = "windows"))]
pub use hyperlocal::UnixConnector;

#[cfg(target_os = "windows")]
pub use tcp::{TcpSocket};

/// Trait for both Unix and TCP sockets.
pub trait Socket{
    ///
    fn new(address: &str) -> Self;

    /// Returns the socket address
    fn address(&self) -> String;

    /// Execute a request to the docker API through a socket connection
    fn request(&mut self, uri: Uri, method: Method, body: Option<String>) -> Option<Value>;
}