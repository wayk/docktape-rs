extern crate docktape;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper;

use docktape::*;
use docktape::unix::{UnixSocket};
use docktape::Docker;

#[test]
fn unix_socket(){

    let socket = UnixSocket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());

    let containers = docker.get_containers();
    match containers{
        Some(containers) =>{
            println!("\nContainers:");
            for container in containers{
                let cont = docker.inspect_container(container.name()).unwrap();
                println!("Name: {}, Running: {}", cont.name, cont.running);
            }
        },
        None =>{

        }
    }

    let images = docker.get_images();
    match images{
        Some(images) =>{
            println!("\nImages:");
            for image in images{
                if let Some(name) = image.repo_tags_name(){
                    if let Some(img) = docker.inspect_image(&name){
                        println!("ID: {}, Name: {}", img.id(), name);
                    }
                    else{
                        println!("Cannot get image {}!", name);
                    }
                }
            }
        },
        None =>{

        }
    }
}