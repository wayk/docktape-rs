/// Struct representing a Docker image with some of its fields
pub struct Image{
    pub id: String,
    pub repo_tags: Option<Vec<String>>
}

impl Image{
    /// Returns the image ID
    pub fn id(&self) -> String{
        self.id.clone().replace("\"", "")
    }

    /// Returns the container tags
    pub fn repo_tags(&self) -> Option<Vec<String>>
    {
        let mut tags = Vec::new();
        for tag in self.repo_tags.clone().unwrap(){
            tags.push(tag.replace("\"", ""));
        }

        Some(tags)
    }
}