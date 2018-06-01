extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;
extern crate serde_json;

pub mod container;
pub mod image;
pub mod unix;
pub mod tcp;
pub mod docker;

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
    fn do_work(&mut self, method: Method, uri: Uri) -> Option<Value>;
}