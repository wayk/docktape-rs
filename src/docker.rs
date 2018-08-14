#[cfg(not(target_os = "windows"))]
use hyperlocal::Uri as HyperlocalUri;
use hyper::Uri as HyperUri;
use hyper::Method::*;
use serde_json::Value;
use percent_encoding::{utf8_percent_encode};
use percent_encoding::SIMPLE_ENCODE_SET;
use volume::Volume;
use image::Image;
use container::Container;
use network::Network;
use socket::Socket;
use std::fs::File;
use std::io::Read;
use hyper::Body;

define_encode_set! {
    pub QUERY_ENCODE_SET = [SIMPLE_ENCODE_SET] | {' ', '"', '#', '<', '>', '/', ':'}
}

/// Struct representing a Docker object with its socket
pub struct Docker{
    socket: Socket
}

impl Docker{
    ///
    pub fn new(socket: Socket) -> Self{
        Docker{
            socket
        }
    }

    /// Create the URI
    #[cfg(not(target_os = "windows"))]
    fn create_uri(&self, path: &str) -> HyperUri{
        if self.socket.is_unix(){
            return HyperlocalUri::new(self.socket.address(), path).into();
        }

        format!("{}{}", self.socket.address(), path).parse().unwrap()
    }

    /// Create the URI
    #[cfg(target_os = "windows")]
    fn create_uri(&self, path: &str) -> HyperUri{
        format!("{}{}", self.socket.address(), path).parse().unwrap()
    }

    /// Returns Docker informations
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.get_info(){
    ///       Some(info) =>{
    ///           println!("Dock info: {}", info);
    ///       },
    ///       None =>{
    ///           println!("No info! (Docker is offline?)");
    ///       }
    ///   }
    ///
    /// ```
    pub fn get_info(&mut self) -> Option<Value>{
        let uri = self.create_uri("/info");

        match self.socket.request(uri, Get, None) {
            Some(info) => {
               Some(info)
            },
            None =>{
                None
            }
        }
    }

    /// Creates a Docker image from a public image
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.create_image_from_image("fedora:latest", ""){
    ///        Some(msg) =>{
    ///            println!("Error message: {:?}", msg["message"]);
    ///        }
    ///        None =>{
    ///            println!("Image created.");
    ///        }
    ///    }
    ///
    /// ```
    pub fn create_image_from_image(&mut self, name: &str, repo: &str, platform: &str) -> Option<Value>{
        let container_name = &format!("/images/create?fromImage={}&repo={}&platform={}", name, repo, platform);
        let uri = self.create_uri(container_name);

        match self.socket.request(uri, Post, None){
            Some(body) =>{
                Some(body)
            },
            None =>{
                None
            }
        }
    }

    /// Returns a vec of Docker images
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.get_images(){
    ///       Some(images) =>{
    ///           println!("There are {} image(s).", images.len());
    ///       },
    ///       None =>{
    ///           println!("There are no images.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn get_images(&mut self) -> Option<Vec<Image>>{
        let uri = self.create_uri("/images/json");

        match self.socket.request(uri, Get, None) {
            Some(imgs) => {
                let mut images = Vec::new();
                let arr_images: &Vec<Value> = imgs.as_array().unwrap();
                for c in arr_images{
                    let mut tags = Vec::new();
                    for tag in c["RepoTags"].as_array().unwrap(){
                        tags.push(tag.to_string());
                    }
                    images.push(Image{
                        id: c["Id"].to_string(),
                        repo_tags: Some(tags) });
                }

                Some(images)
            },
            None =>{
                None
            }
        }
    }

    /// Inspects a Docker image
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.inspect_image("fedora:latest"){
    ///       Some(img) => {
    ///           println!("Image id: {}.", img.id());
    ///       }
    ///       None => {
    ///           println!("Cannot get image.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn inspect_image(&mut self, image: &str) -> Option<Image>{
        let image = utf8_percent_encode(image.as_ref(), QUERY_ENCODE_SET).to_string();
        let image_name: String = format!("/images/{}/json", image);
        let uri = self.create_uri(image_name.as_str());

        match self.socket.request(uri, Get, None) {
            Some(image) => {
                match image["RepoTags"].as_array(){
                    Some(tags) =>{
                        let mut ts = Vec::new();
                        for tag in tags{
                            ts.push(tag.to_string());
                        }
                        Some(
                            Image{
                                id: image["Id"].to_string(),
                                repo_tags: Some(ts)
                            }
                        )
                    },
                    None =>{
                        None
                    }
                }
            },
            None =>{
                None
            }
        }
    }

    /// Deletes a Docker image
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.delete_image("fedora:latest"){
    ///       Some(_) =>{
    ///           println!("Image deleted.");
    ///       }
    ///       None =>{
    ///           println!("Error while deleting image.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn delete_image(&mut self, name: &str) -> Option<Value>{
        let path = &format!("/images/{}", name);
        let uri = self.create_uri(path);
        match self.socket.request(uri, Delete, None) {
            Some(message) => {
                Some(message)
            },
            None =>{
                None
            }
        }
    }

    /// Creates a Docker container
    /// # Example
    ///
    /// ```rust,no_run
    ///  let body = json!({
    ///       "Image": "fedora:latest",
    ///       "Cmd": ["echo"]
    ///   });
    ///
    ///   match docker.create_container(&body.to_string(), "my_container"){
    ///       Some(_) =>{
    ///           println!("Container created.");
    ///       },
    ///       None =>{
    ///           println!("Error while creating container.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn create_container(&mut self, body_str: String, name: &str) -> Option<Container>{
        let container_name = &format!("/containers/create?name={}", name);
        let uri = self.create_uri(container_name);

        let body = Body::from(body_str);

        match self.socket.request(uri, Post, Some(body)){
            Some(container) =>{
                Some(Container{
                    id: container["Id"].to_string(),
                    name: container["Names"][0].to_string(),
                    image: container["Image"].to_string(),
                    running: "false".to_string()
                })
            },
            None =>{
                None
            }
        }
    }

    /// Returns a vec of Docker containers
    /// # Example
    ///
    /// ```rust,no_run
    /// match docker.get_containers(){
    ///       Some(containers) =>{
    ///           println!("There are {} container(s).", containers.len());
    ///       },
    ///       None =>{
    ///           println!("There are no containers.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn get_containers(&mut self) -> Option<Vec<Container>>{
        let uri = self.create_uri("/containers/json");

        match self.socket.request(uri, Get, None) {
            Some(conts) => {
                let mut containers = Vec::new();
                let arr_containers: &Vec<Value> = conts.as_array().unwrap();
                for c in arr_containers{
                    containers.push(Container{
                        id: c["Id"].to_string(),
                        name: c["Names"][0].to_string(),
                        image: c["Image"].to_string(),
                        running: "false".to_string()
                    });
                }

                Some(containers)
            },
            None =>{
                None
            }
        }
    }

    /// Inspects a Docker container
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.inspect_container("my_container"){
    ///       Some(container) =>{
    ///           println!("Container id: {}.", container.id());
    ///       }
    ///       None =>{
    ///           println!("Container can't be found.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn inspect_container(&mut self, container: &str) -> Option<Container>{
        let container_name =
                format!(
                    "/containers/{}/json",
                    utf8_percent_encode(container.as_ref(), QUERY_ENCODE_SET).to_string());

        let uri = self.create_uri(container_name.as_str());

        match self.socket.request(uri, Get, None) {
            Some(container) => {
                if container["Id"].to_string() == "null" {
                    None
                }
                else {
                    Some(Container {
                            id: container["Id"].to_string(),
                            name: container["Name"].to_string(),
                            image: container["Config"]["Image"].to_string(),
                            running: container["State"]["Running"].to_string(),
                    })
                }
            },
            None =>{
                None
            }
        }
    }

    /// Starts a Docker container
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.start_container("my_container"){
    ///       Some(msg) =>{
    ///           println!("Error message: {:?}.", msg["message"]);
    ///       }
    ///       None =>{
    ///           println!("Container started.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn start_container(&mut self, container: &str) -> Option<Value>{
        let path = &format!("/containers/{}/start", container);
        let uri = self.create_uri(path);
        match self.socket.request(uri, Post, None){
            Some(body) =>{
                Some(body)
            },
            None =>{
                None
            }
        }
    }

    /// Stops a Docker container
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.stop_container("my_container"){
    ///       Some(msg) =>{
    ///           println!("Error message: {:?}.", msg["message"]);
    ///       }
    ///       None =>{
    ///           println!("Container stopped.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn stop_container(&mut self, container: &str) -> Option<Value>{
        let path = &format!("/containers/{}/stop", container);
        let uri = self.create_uri(path);
        match self.socket.request(uri, Post, None){
            Some(body) =>{
                Some(body)
            },
            None =>{
                None
            }
        }
    }

    /// Restarts a Docker container
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.restart_container("my_container"){
    ///       Some(msg) =>{
    ///           println!("Error message: {:?}.", msg["message"]);
    ///       }
    ///       None =>{
    ///           println!("Container restarted.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn restart_container(&mut self, container: &str) -> Option<Value>{
        let path = &format!("/containers/{}/restart", container);
        let uri = self.create_uri( path);
        match self.socket.request(uri, Post, None){
            Some(body) =>{
                Some(body)
            },
            None =>{
                None
            }
        }
    }

    /// Deletes a Docker container
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.delete_container("my_container"){
    ///       Some(msg) =>{
    ///           println!("Error message: {:?}", msg["message"]);
    ///       }
    ///       None =>{
    ///           println!("Container deleted.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn delete_container(&mut self, container: &str) -> Option<Value>{
        let path = &format!("/containers/{}", container);
        let uri = self.create_uri(path);
        match self.socket.request(uri, Delete, None){
            Some(message) =>{
                Some(message)
            },
            None =>{
                None
            }
        }
    }

    /// Put a TAR file in a path in the Docker container
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.delete_container("my_container"){
    ///       Some(msg) =>{
    ///           println!("Error message: {:?}", msg["message"]);
    ///       }
    ///       None =>{
    ///           println!("Container deleted.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn put_container(&mut self, container: &str, path: &str, tar_path: &str) -> Option<Value>{
        let container = utf8_percent_encode(container.as_ref(), QUERY_ENCODE_SET).to_string();
        let path = utf8_percent_encode(path.as_ref(), QUERY_ENCODE_SET).to_string();

        let path = &format!("/containers/{}/archive?path={}", container, path);
        let uri = self.create_uri(path);
        let mut file = File::open(tar_path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let body = Body::from(buffer);

        match self.socket.request(uri, Put, Some(body)){
            Some(message) =>{
                Some(message)
            },
            None =>{
                None
            }
        }
    }

    /// Creates a Docker network
    /// # Example
    ///
    /// ```rust,no_run
    ///  let body = json!({
    ///       "Name": "my_network",
    ///       "CheckDuplicate": true
    ///   });
    ///
    ///   match docker.create_network(&body.to_string()) {
    ///       Some(_) =>{
    ///           println!("Network created.");
    ///       }
    ///       None =>{
    ///           println!("Error while creating network.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn create_network(&mut self, body_str: String) -> Option<Network>{
        let uri = self.create_uri("/networks/create");

        let body = Body::from(body_str);

        match self.socket.request(uri, Post, Some(body)){
            Some(network) =>{
                println!("Create network: {}", network);
                Some(Network{
                    id: network["Id"].to_string(),
                    name: network["Name"].to_string(),
                })
            },
            None =>{
                None
            }
        }
    }

    /// Returns a vec of Docker networks
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.get_networks(){
    ///       Some(networks) =>{
    ///           println!("There are {} network(s).", networks.len());
    ///       },
    ///       None =>{
    ///           println!("There are no networks.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn get_networks(&mut self) -> Option<Vec<Network>>{
        let uri = self.create_uri("/networks");

        match self.socket.request(uri, Get, None) {
            Some(ntws) => {
                let mut networks = Vec::new();
                let arr_networks: &Vec<Value> = ntws.as_array().unwrap();
                for c in arr_networks{
                    networks.push(Network{
                        id: c["Id"].to_string(),
                        name: c["Name"][0].to_string()
                    });
                }

                Some(networks)
            },
            None =>{
                None
            }
        }
    }

    /// Inspects a Docker network
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.inspect_network("my_network"){
    ///       Some(network) => {
    ///           println!("Network id: {}.", network.id());
    ///       }
    ///       None => {
    ///           println!("Cannot get network.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn inspect_network(&mut self, name: &str) -> Option<Network>{
        let path = &format!("/networks/{}", name);
        let uri = self.create_uri(path);
        match self.socket.request(uri, Get, None) {
            Some(network) => {
                if !network["message"].is_null(){
                    return None;
                }

                return Some(Network{
                    id: network["Id"].to_string(),
                    name: network["Name"].to_string(),
                });
            },
            None =>{
               return None;
            }
        }
    }

    /// Deletes a Docker network
    /// # Example
    ///
    /// ```rust,no_run
    ///   match docker.delete_network("my_network"){
    ///       Some(msg) =>{
    ///           println!("Error message: {:?}.", msg["message"]);
    ///       }
    ///       None =>{
    ///           println!("Network deleted.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn delete_network(&mut self, name: &str) -> Option<Value>{
        let path = &format!("/networks/{}", name);
        let uri = self.create_uri(path);
        match self.socket.request(uri, Delete, None) {
            Some(message) => {
                Some(message)
            },
            None =>{
                None
            }
        }
    }

    /// Creates a Docker volume
    /// # Example
    ///
    /// ```rust,no_run
    ///    let body = json!({
    ///       "Name": "my_volume"
    ///   });
    ///
    ///   match docker.create_volume(&body.to_string()){
    ///       Some(_) =>{
    ///           println!("Volume created.");
    ///       }
    ///       None =>{
    ///           println!("Error while creating volume.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn create_volume(&mut self, body_str: String) -> Option<Volume>{
        let uri = self.create_uri("/volumes/create");

        let body = Body::from(body_str);

        match self.socket.request(uri, Post, Some(body)){
            Some(volume) =>{
                Some(Volume{
                    name: volume["Name"].to_string(),
                    mountpoint: volume["Mountpoint"].to_string(),
                })
            },
            None =>{
                None
            }
        }
    }

    /// Returns a vec of Docker volumes
    /// # Example
    ///
    /// ```rust,no_run
    ///   match docker.get_volumes(){
    ///       Some(volumes) =>{
    ///           println!("There are {} volume(s).", volumes.len());
    ///       },
    ///       None =>{
    ///           println!("There are no volumes.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn get_volumes(&mut self) -> Option<Vec<Volume>>{
        let uri = self.create_uri("/volumes");

        match self.socket.request(uri, Get, None) {
            Some(vols) => {
                let mut volumes = Vec::new();
                let arr_volumes: &Vec<Value> = vols["Volumes"].as_array().unwrap();
                for v in arr_volumes{
                    volumes.push(Volume{
                        name: v["Name"].to_string(),
                        mountpoint: v["Mountpoint"].to_string()
                    });
                }

                Some(volumes)
            },
            None =>{
                None
            }
        }
    }

    /// Inspects a Docker volume
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.inspect_volume("my_volume"){
    ///       Some(volume) =>{
    ///           println!("Volume mountpoint: {}.", volume.mountpoint());
    ///       }
    ///       None =>{
    ///           println!("Volume can't be found.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn inspect_volume(&mut self, name: &str) -> Option<Volume>{
        let path = &format!("/volumes/{}", name);
        let uri = self.create_uri(path);
        match self.socket.request(uri, Get, None) {
            Some(volume) => {
                Some(Volume{
                    name: volume["Name"].to_string(),
                    mountpoint: volume["Mountpoint"].to_string(),
                })
            },
            None =>{
                None
            }
        }
    }

    /// Deletes a Docker volume
    /// # Example
    ///
    /// ```rust,no_run
    ///  match docker.delete_volume("my_volume"){
    ///       Some(msg) =>{
    ///           println!("Error message: {:?}", msg["message"]);
    ///       }
    ///       None =>{
    ///           println!("Volume deleted.");
    ///       }
    ///   }
    ///
    /// ```
    pub fn delete_volume(&mut self, name: &str) -> Option<Value>{
        let path = &format!("/volumes/{}", name);
        let uri = self.create_uri(path);
        match self.socket.request(uri, Delete, None) {
            Some(message) => {
                Some(message)
            },
            None =>{
                None
            }
        }
    }
}