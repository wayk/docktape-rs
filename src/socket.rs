#[cfg(not(target_os = "windows"))]
use hyperlocal::{UnixConnector};
use tokio_core::reactor::Core;
use hyper::{Client, Uri, Request, Method};
use hyper::client::HttpConnector;
use hyper::header::ContentType;
use futures::Future;
use futures::Stream;
use serde_json;
use serde_json::Value;
use std::io::{self};
use utils;
use hyper::Body;

///Socket
#[derive(Clone)]
pub struct Socket{
    /// Socket address
    pub address: String
}

impl Socket{
    /// Create new socket
    pub fn new(address: &str) -> Self{
        Socket{
            address: address.to_string()
        }
    }

    /// Returns the Socket address
    pub fn address(&self) -> String{
        self.address.clone()
    }

    /// Returns if the Socket is a Unix one
    pub fn is_unix(&self) -> bool{
        match utils::is_http_scheme(&self.address()){
            Some(scheme) =>{
                if scheme == "http"{
                    false
                }
                else {
                    true
                }
            },
            None => {
                true
            }
        }
    }

    /// Execute the request on the client
    #[cfg(target_os = "windows")]
    pub fn request<T>(&mut self, uri: Uri, method: Method, body: Option<T>) -> Option<Value>{
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure().connector(HttpConnector::new(4, &handle)).build(&core.handle());

        let mut request = Request::new(method, uri);
        request.headers_mut().set(ContentType::json());
        if let Some(b) = body{
            request.set_body(b);
        }

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
            Err(e)=>{
                error!("Error message: {}", e);
                None
            }
        }
    }

    /// Execute the request on the client
    #[cfg(not(target_os = "windows"))]
    pub fn request(&mut self, uri: Uri, method: Method, body: Option<Body>) -> Option<Value>{
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        if self.is_unix() {
            let client = Client::configure().connector(UnixConnector::new(handle)).build(&core.handle());
            let mut request = Request::new(method, uri);
            request.headers_mut().set(ContentType::json());
            if let Some(b) = body{
                request.set_body(b);
            }

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
                Err(e)=>{
                    error!("Error message: {}", e);
                    None
                }
            }
        }
        else{
            let client = Client::configure().connector(HttpConnector::new(4, &handle)).build(&core.handle());
            let mut request = Request::new(method, uri);
            request.headers_mut().set(ContentType::json());
            if let Some(b) = body{
                request.set_body(b);
            }

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
                Err(e)=>{
                    error!("Error message: {}", e);
                    None
                }
            }
        }
    }
}