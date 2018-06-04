use hyper::{Client};
use hyperlocal::{Uri, UnixConnector};
use tokio_core::reactor::Core;
use futures::Future;
use std::io::{self};
use futures::Stream;
use hyper::Request;
use hyper::Method;
use serde_json;
use serde_json::Value;
use Socket;
use hyper::Get;
use hyper::Post;
use hyper::Delete;
use hyper::header::ContentType;

#[derive(Clone)]
pub struct UnixSocket{
    pub address: String
}

impl Socket for UnixSocket{
    fn address(&self) -> String{
        self.address.clone()
    }

    fn new(address: &str) -> Self{
        UnixSocket{
            address: address.to_string()
        }
    }

    fn request_get(&mut self, uri: Uri) -> Option<Value>{
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure().connector(UnixConnector::new(handle)).build(&core.handle());

        let work = client.request(Request::new(Get, uri.into())).and_then(|res| {
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

    fn request_post(&mut self, uri: Uri, body: String) -> Option<Value>{
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure().connector(UnixConnector::new(handle)).build(&core.handle());
        let mut request = Request::new(Post, uri.into());
        request.headers_mut().set(ContentType::json());
        request.set_body(body);

        let work = client.request(request).and_then(|res| {
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

    fn request_delete(&mut self, uri: Uri) -> Option<Value>{
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure().connector(UnixConnector::new(handle)).build(&core.handle());
        let mut request = Request::new(Delete, uri.into());

        let work = client.request(request).and_then(|res| {
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