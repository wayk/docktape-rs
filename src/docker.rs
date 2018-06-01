use Socket;
use hyperlocal::Uri;
use hyper::Method::*;
use image::Image;
use container::Container;
use serde_json::Value;

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

        match self.socket.do_work(Get, uri) {
            Some(conts) => {
                let mut containers = Vec::new();
                let arr_containers: &Vec<Value> = conts.as_array().unwrap();
                for c in arr_containers{
                    containers.push(Container{
                        id: c["Id"].to_string(),
                        name: c["Names"][0].to_string(),
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

        match self.socket.do_work(Get, uri) {
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
                        repo_tags: Some(tags),
                    });
                }

                Some(images)
            },
            None =>{
                None
            }
        }
    }

    pub fn inspect_container(&self, _container: &str){

    }

    pub fn inspect_image(&self, _image: &str){

    }
}