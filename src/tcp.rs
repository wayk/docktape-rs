use tokio_core::reactor::Core;
use hyper::{Client, Uri, Request, Method};
use hyper::client::HttpConnector;
use hyper::header::ContentType;
use futures::Future;
use futures::Stream;
use serde_json;
use serde_json::Value;
use std::io::{self};
use Socket;

/// Unix socket
#[derive(Clone)]
pub struct TcpSocket{
    /// Socket address
    pub address: String
}

impl Socket for TcpSocket{
    ///
    fn new(address: &str) -> Self{
        TcpSocket{
            address: address.to_string()
        }
    }

    ///
    fn address(&self) -> String{
        self.address.clone()
    }

    ///
    fn request(&mut self, uri: Uri, method: Method, body: Option<String>) -> Option<Value>{
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
                if item["message"].to_string() == "null"{
                    Some(item)
                }
                    else{
                        error!("Message: {}", item["message"].to_string());
                        None
                    }
            },
            Err(_)=>{
                None
            }
        }
    }
}