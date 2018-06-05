/// Struct representing a Docker Volume with some of its fields
pub struct Volume{
    pub name: String,
    pub mountpoint: String
}

impl Volume{
    /// Returns the volume ID
    pub fn name(&self) -> String{
        self.name.clone().replace("\"", "")
    }

    /// Returns the volume mountpoint
    pub fn mountpoint(&self) -> String{
        self.mountpoint.clone().replace("\"", "")
    }
}