extern crate docktape;
extern crate tokio_core;
#[macro_use]
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

    match docker.create_network("den-net") {
        Some(network) => {
            println!("Network ID: {:?}, NAME: {}", network.id(), network.name());
        },
        None => {

        }
    }

    match docker.inspect_network("den-net") {
        Some(network) => {
            println!("Network ID: {:?}, NAME: {}", network.id(), network.name());
        },
        None => {

        }
    }

    //docker.delete_network("newnetwork");

    let body = json!({
        "NetworkMode": "den-net",
        "Image": "devolutions/denrouter-rs:0.1.6-dev",
        "PortBindings": {
            "4491/tcp": [{ "HostPort": "4491" }]
        }
    });

    match docker.create_container(&body.to_string(), "router") {
        Some(container) => {
            println!("Result: {:?}", container.name());
        },
        None => {

        }
    }

    let body = json!({
        "NetworkMode": "den-net",
        "Image": "devolutions/waykden-rs:0.1.8-dev",
        "Cmd": ["-ltrace", "-u", "ws://router:4491"]
    });

    match docker.create_container(&body.to_string(), "den") {
        Some(container) => {
            println!("Result: {:?}", container.name());
        },
        None => {

        }
    }

    docker.start_container("router");
    docker.start_container("den");
}