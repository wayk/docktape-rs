# Docktape
[![Docktape doc badge](https://docs.rs/docktape/badge.svg)](https://docs.rs/docktape/)

## Unix Socket initialization
```rust
// Unix socket
let socket = UnixSocket::new("<Path to your socket>");

//TCP socket (Not supported yet)
```

## Initialize Docker
```rust
let mut docker = Docker::new(socket.clone());
```

## Make call to Docker API
```rust
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

...
```

