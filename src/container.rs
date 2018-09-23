/// Struct representing a Docker container with some of its fields
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub running: String,
}

impl Container {
    /// Returns the container ID
    pub fn id(&self) -> String {
        self.id.clone().replace("\"", "")
    }

    /// Returns the container name
    pub fn name(&self) -> String {
        self.name.clone().replace("\"", "")
    }

    /// Returns the container image
    pub fn image(&self) -> String {
        self.image.clone().replace("\"", "")
    }

    /// Returns if the container is running
    pub fn running(&self) -> String {
        self.running.clone().replace("\"", "")
    }
}