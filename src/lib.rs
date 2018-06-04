extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;

pub mod container;
pub mod image;
pub mod network;
pub mod unix;
pub mod tcp;
pub mod docker;
pub mod utils;

pub use futures::Stream;
pub use futures::Future;
pub use hyper::{Client, Result, Request};
pub use hyperlocal::{Uri, UnixConnector};
pub use tokio_core::reactor::Core;
pub use std::io::{self, Write};
pub use serde_json::Value;
pub use docker::Docker;
pub use hyper::Method;

pub trait Socket{
    fn address(&self) -> String;
    fn new(address: &str) -> Self;
    fn request_get(&mut self, uri: Uri) -> Option<Value>;
    fn request_post(&mut self, uri: Uri, body: String) -> Option<Value>;
    fn request_delete(&mut self, uri: Uri) -> Option<Value>;
}