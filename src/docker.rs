use Socket;
use hyperlocal::Uri;
use hyper::Method::*;
use image::Image;
use container::Container;
use serde_json::Value;
use utils;
use network::Network;

pub struct Docker<T: Socket>{
    socket: T
}

impl<T> Docker<T> where T: Socket{
    pub fn new(socket: T) -> Self{
        Docker{
            socket
        }
    }

    pub fn get_containers(&mut self) -> Option<Vec<Container>>{
        let uri = Uri::new(self.socket.address(), "/containers/json");

        match self.socket.request_get(uri) {
            Some(conts) => {
                let mut containers = Vec::new();
                let arr_containers: &Vec<Value> = conts.as_array().unwrap();
                for c in arr_containers{
                    containers.push(Container{
                        id: c["Id"].to_string(),
                        name: c["Names"][0].to_string(),
                        image: c["Image"].to_string(),
                        running: true
                    });
                }

                Some(containers)
            },
            None =>{
                None
            }
        }
    }

    pub fn get_images(&mut self) -> Option<Vec<Image>>{
        let uri = Uri::new(self.socket.address(), "/images/json");

        match self.socket.request_get(uri) {
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

    pub fn inspect_container(&mut self, container: String) -> Option<Container>{
        let mut container_name: String = String::new();
        if(container.starts_with("/")){
            container_name = format!("/containers{}/json", container);
        }
        else{
            container_name = format!("/containers/{}/json", container);
            println!("Container name: {}", container_name);
        }

        let uri = Uri::new(self.socket.address(), container_name.as_str());

        match self.socket.request_get(uri) {
            Some(container) => {
                Some(
                    Container{
                        id: container["Id"].to_string(),
                        name: container["Name"].to_string(),
                        image: container["Config"]["Image"].to_string(),
                        running: true,
                    }
                )
            },
            None =>{
                None
            }
        }
    }

    pub fn inspect_image(&mut self, image: &str) -> Option<Image>{
        let mut image_name: String = format!("/images/{}/json", image);
        let uri = Uri::new(self.socket.address(), image_name.as_str());

        match self.socket.request_get(uri) {
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

    pub fn create_network(&mut self, name: &str) -> Option<Network>{
        let uri = Uri::new(self.socket.address(), "/networks/create");
        let body = json!({
          "Name": name
        });

        match self.socket.request_post(uri, body.to_string()){
            Some(network) =>{
                Some(Network{
                    id: network["Id"].to_string(),
                    name: name.to_string(),
                })
            },
            None =>{
                None
            }
        }
    }

    pub fn inspect_network(&mut self, name: &str) -> Option<Network>{
        let path = &format!("/networks/{}", name);
        let uri = Uri::new(self.socket.address(), path);
        let body = json!({});
        match self.socket.request_get(uri) {
            Some(network) => {
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

    pub fn delete_network(&mut self, name: &str) -> Option<String>{
        let path = &format!("/networks/{}", name);
        let uri = Uri::new(self.socket.address(), path);
        let body = json!({});
        match self.socket.request_delete(uri) {
            Some(success) => {
                Some(success.to_string())
            },
            None =>{
                None
            }
        }
    }

    pub fn create_container(&mut self, body: &str, name: &str) -> Option<Container>{
        let container_name = &format!("/containers/create?name={}", name);
        let uri = Uri::new(self.socket.address(), container_name);

        match self.socket.request_post(uri, body.to_string()){
            Some(body) =>{
                let result_id: &str = &utils::clean_string(&body["Id"].to_string());
                self.inspect_container(result_id.to_string())
            },
            None =>{
                None
            }
        }
    }

    pub fn start_container(&mut self, container: &str){
        let path = &format!("/containers/{}/start", container);
        let uri = Uri::new(self.socket.address(), path);
        let body = json!({});
        self.socket.request_post(uri, body.to_string());
    }

    pub fn stop_container(&mut self, container: &str){
        let path = &format!("/containers/{}/stop", container);
        let uri = Uri::new(self.socket.address(), path);
        let body = json!({});
        self.socket.request_post(uri, body.to_string());
    }

    pub fn restart_container(&mut self, container: &str){
        let path = &format!("/containers/{}/restart", container);
        let uri = Uri::new(self.socket.address(), path);
        let body = json!({});
        self.socket.request_post(uri, body.to_string());
    }
}