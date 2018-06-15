extern crate docktape;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper;

use docktape::*;
use docktape::Docker;

#[test]
fn put_container_test() {
    let socket = Socket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());
    docker.put_container("router", "/tmp", "archive.tar.gz");
}

#[test]
fn info_test() {
    let socket = Socket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());

    match docker.get_info(){
        Some(info) =>{
            println!("Dock info: {}", info);
        },
        None =>{
            println!("No info! (Docker is offline?)");
        }
    }
}

#[test]
fn images_test() {
    let socket = Socket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());

    match docker.create_image_from_image("fedora:latest", "", "linux"){
        Some(msg) =>{
            if msg["message"].to_string() != "null" {
                println!("Error message: {:?}", msg["message"]);
            }
            else{
                println!("Image created.");
            }
        }
        None =>{
            println!("Failed to create image.");
        }
    }

    match docker.get_images(){
        Some(images) =>{
            println!("There are {} image(s).", images.len());
        },
        None =>{
            println!("There are no images.");
        }
    }

    match docker.inspect_image("fedora:latest"){
        Some(img) => {
            println!("Image id: {}.", img.id());
        }
        None => {
            println!("Cannot get image.");
        }
    }

    match docker.delete_image("fedora:latest"){
        Some(msg) =>{
            if msg["message"].to_string() != "null" {
                println!("Error message: {:?}", msg["message"]);
            }
            else{
                println!("Image deleted.");
            }
        }
        None =>{
            println!("Error while deleting image.");
        }
    }
}
