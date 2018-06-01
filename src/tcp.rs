use hyper::{Client};
use hyperlocal::{Uri, UnixConnector};
use tokio_core::reactor::Core;
use futures::Future;
pub use std::io::{self, Write};
use futures::Stream;
use hyper::Request;
use hyper::Method;
use serde_json;
use serde_json::Value;
use Socket;

#[derive(Clone)]
pub struct TcpSocket{
    pub address: String
}

impl Socket for TcpSocket{
    fn address(&self) -> String{
        self.address.clone()
    }

    fn new(address: &str) -> Self{
        TcpSocket{
            address: address.to_string()
        }
    }

    fn do_work(&mut self, method: Method, uri: Uri) -> Option<Value>{
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure().connector(UnixConnector::new(handle)).build(&core.handle());

        let work = client.request(Request::new(method, uri.into())).and_then(|res| {
            res.body().concat2().and_then(move |body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                })?;
                Ok(v)
            })
        });

        match core.run(work){
            Ok(item) =>{
                Some(item)
            },
            Err(_)=>{
                None
            }
        }
    }
}