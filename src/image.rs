/// Struct representing a Docker image with some of its fields
pub struct Image {
    pub id: String,
    pub repo_tags: Option<Vec<String>>,
    pub repo_digests: Option<Vec<String>>,
    pub parent: String,
    pub comment: String,
    pub created: String,
    pub container: String,
    pub docker_version: String,
    pub author: String,
    pub architecture: String,
    pub os: String,
    pub size: i64,
    pub virtual_size: i64,
}

impl Image {
    /// Returns the image ID
    pub fn id(&self) -> String {
        self.id.clone().replace("\"", "")
    }

    /// Returns the image tags
    pub fn repo_tags(&self) -> Option<Vec<String>> {
        let mut tags = Vec::new();
        for tag in self.repo_tags.clone().unwrap() {
            tags.push(tag.replace("\"", ""));
        }

        Some(tags)
    }

    /// Returns the image repo digests
    pub fn repo_digests(&self) -> Option<Vec<String>> {
        let mut digests = Vec::new();
        for digest in self.repo_digests.clone().unwrap() {
            digests.push(digest.replace("\"", ""));
        }

        Some(digests)
    }
}