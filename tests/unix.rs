extern crate docktape;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper;

use docktape::*;
use docktape::unix::{UnixSocket};
use docktape::Docker;

#[test]
fn unix_socket() {

    let socket = UnixSocket::new("/var/run/docker.sock");
    let mut docker = Docker::new(socket.clone());

    let containers = docker.get_containers();
    match containers{
        Some(containers) =>{
            println!("\nContainers:");
            for container in containers{
                println!("ID: {} - NAME: {}", container.id, container.name);
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
                println!("ID: {} - TAGS: {:?}", image.id, image.repo_tags);
            }
        },
        None =>{

        }
    }
}