/// Struct representing a Docker network with some of its fields
pub struct Network{
    pub id: String,
    pub name: String
}

impl Network{
    /// Returns the network ID
    pub fn id(&self) -> String{
        self.id.clone().replace("\"", "")
    }

    /// Returns the network name
    pub fn name(&self) -> String{
        self.name.clone().replace("\"", "")
    }
}