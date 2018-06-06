extern crate docktape;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;
extern crate hyper;

use docktape::*;
use docktape::Docker;

#[cfg(not(target_os = "windows"))]
#[test]
fn images_test() {
    let socket = UnixSocket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());

    match docker.create_image_from_image("fedora:latest", ""){
        Some(msg) =>{
            println!("Error message: {:?}", msg["message"]);
        }
        None =>{
            println!("Image created.");
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
        Some(_) =>{
            println!("Image deleted.");
        }
        None =>{
            println!("Error while deleting image.");
        }
    }
}

#[cfg(not(target_os = "windows"))]
#[test]
fn containers_test() {
    let socket = UnixSocket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());

    let body = json!({
        "Image": "fedora:latest",
        "Cmd": ["echo"]
    });

    match docker.create_container(&body.to_string(), "my_container"){
        Some(_) =>{
            println!("Container created.");
        },
        None =>{
            println!("Error while creating container.");
        }
    }

    match docker.start_container("my_container"){
        Some(msg) =>{
            println!("Error message: {:?}.", msg["message"]);
        }
        None =>{
            println!("Container started.");
        }
    }

    match docker.get_containers(){
        Some(containers) =>{
            println!("There are {} container(s).", containers.len());
        },
        None =>{
            println!("There are no containers.");
        }
    }

    match docker.inspect_container("my_container"){
        Some(container) =>{
            println!("Container id: {}.", container.id());
        }
        None =>{
            println!("Container can't be found.");
        }
    }

    match docker.restart_container("my_container"){
        Some(msg) =>{
            println!("Error message: {:?}.", msg["message"]);
        }
        None =>{
            println!("Container restarted.");
        }
    }

    match docker.stop_container("my_container"){
        Some(msg) =>{
            println!("Error message: {:?}.", msg["message"]);
        }
        None =>{
            println!("Container stopped.");
        }
    }

    match docker.delete_container("my_container"){
        Some(msg) =>{
            println!("Error message: {:?}", msg["message"]);
        }
        None =>{
            println!("Container deleted.");
        }
    }
}

#[cfg(not(target_os = "windows"))]
#[test]
fn networks_test(){
    let socket = UnixSocket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());

    let body = json!({
        "Name": "my_network",
        "CheckDuplicate": true
    });

    match docker.create_network(&body.to_string()) {
        Some(_) =>{
            println!("Network created.");
        }
        None =>{
            println!("Error while creating network.");
        }
    }

    match docker.get_networks(){
        Some(networks) =>{
            println!("There are {} network(s).", networks.len());
        },
        None =>{
            println!("There are no networks.");
        }
    }

    match docker.inspect_network("my_network"){
        Some(network) => {
            println!("Network id: {}.", network.id());
        }
        None => {
            println!("Cannot get network.");
        }
    }

    match docker.delete_network("my_network"){
        Some(msg) =>{
            println!("Error message: {:?}.", msg["message"]);
        }
        None =>{
            println!("Network deleted.");
        }
    }
}

#[cfg(not(target_os = "windows"))]
#[test]
fn volumes_test() {
    let socket = UnixSocket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());

    let body = json!({
        "Name": "my_volume"
    });

    match docker.create_volume(&body.to_string()){
        Some(_) =>{
            println!("Volume created.");
        }
        None =>{
            println!("Error while creating volume.");
        }
    }

    match docker.get_volumes(){
        Some(volumes) =>{
            println!("There are {} volume(s).", volumes.len());
        },
        None =>{
            println!("There are no volumes.");
        }
    }

    match docker.inspect_volume("my_volume"){
        Some(volume) =>{
            println!("Volume mountpoint: {}.", volume.mountpoint());
        }
        None =>{
            println!("Volume can't be found.");
        }
    }

    match docker.delete_volume("my_volume"){
        Some(msg) =>{
            println!("Error message: {:?}", msg["message"]);
        }
        None =>{
            println!("Volume deleted.");
        }
    }
}

#[cfg(target_os = "windows")]
#[test]
fn images_test() {
    let socket = TcpSocket::new("http://localhost:2375");
    let mut docker = Docker::new(socket.clone());

    match docker.create_image_from_image("fedora:latest", ""){
        Some(msg) =>{
            println!("Error message: {:?}", msg["message"]);
        }
        None =>{
            println!("Image created.");
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
        Some(_) =>{
            println!("Image deleted.");
        }
        None =>{
            println!("Error while deleting image.");
        }
    }
}

#[cfg(target_os = "windows")]
#[test]
fn containers_test() {
    let socket = TcpSocket::new("http://localhost:2375");
    let mut docker = Docker::new(socket.clone());

    let body = json!({
        "Image": "fedora:latest",
        "Cmd": ["echo"]
    });

    match docker.create_container(&body.to_string(), "my_container"){
        Some(_) =>{
            println!("Container created.");
        },
        None =>{
            println!("Error while creating container.");
        }
    }

    match docker.start_container("my_container"){
        Some(msg) =>{
            println!("Error message: {:?}.", msg["message"]);
        }
        None =>{
            println!("Container started.");
        }
    }

    match docker.get_containers(){
        Some(containers) =>{
            println!("There are {} container(s).", containers.len());
        },
        None =>{
            println!("There are no containers.");
        }
    }

    match docker.inspect_container("my_container"){
        Some(container) =>{
            println!("Container id: {}.", container.id());
        }
        None =>{
            println!("Container can't be found.");
        }
    }

    match docker.restart_container("my_container"){
        Some(msg) =>{
            println!("Error message: {:?}.", msg["message"]);
        }
        None =>{
            println!("Container restarted.");
        }
    }

    match docker.stop_container("my_container"){
        Some(msg) =>{
            println!("Error message: {:?}.", msg["message"]);
        }
        None =>{
            println!("Container stopped.");
        }
    }

    match docker.delete_container("my_container"){
        Some(msg) =>{
            println!("Error message: {:?}", msg["message"]);
        }
        None =>{
            println!("Container deleted.");
        }
    }
}

#[cfg(target_os = "windows")]
#[test]
fn networks_test(){
    let socket = TcpSocket::new("http://localhost:2375");
    let mut docker = Docker::new(socket.clone());

    let body = json!({
        "Name": "my_network",
        "CheckDuplicate": true
    });

    match docker.create_network(&body.to_string()) {
        Some(_) =>{
            println!("Network created.");
        }
        None =>{
            println!("Error while creating network.");
        }
    }

    match docker.get_networks(){
        Some(networks) =>{
            println!("There are {} network(s).", networks.len());
        },
        None =>{
            println!("There are no networks.");
        }
    }

    match docker.inspect_network("my_network"){
        Some(network) => {
            println!("Network id: {}.", network.id());
        }
        None => {
            println!("Cannot get network.");
        }
    }

    match docker.delete_network("my_network"){
        Some(msg) =>{
            println!("Error message: {:?}.", msg["message"]);
        }
        None =>{
            println!("Network deleted.");
        }
    }
}

#[cfg(target_os = "windows")]
#[test]
fn volumes_test() {
    let socket = TcpSocket::new("http://localhost:2375");
    let mut docker = Docker::new(socket.clone());

    let body = json!({
        "Name": "my_volume"
    });

    match docker.create_volume(&body.to_string()){
        Some(_) =>{
            println!("Volume created.");
        }
        None =>{
            println!("Error while creating volume.");
        }
    }

    match docker.get_volumes(){
        Some(volumes) =>{
            println!("There are {} volume(s).", volumes.len());
        },
        None =>{
            println!("There are no volumes.");
        }
    }

    match docker.inspect_volume("my_volume"){
        Some(volume) =>{
            println!("Volume mountpoint: {}.", volume.mountpoint());
        }
        None =>{
            println!("Volume can't be found.");
        }
    }

    match docker.delete_volume("my_volume"){
        Some(msg) =>{
            println!("Error message: {:?}", msg["message"]);
        }
        None =>{
            println!("Volume deleted.");
        }
    }
}