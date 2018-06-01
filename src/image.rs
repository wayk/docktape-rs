pub struct Image{
    pub id: String,
    pub repo_tags: Option<Vec<String>>
}

impl Image{
    pub fn id(&self) -> String{
        self.id.clone().replace("\"", "")
    }

    pub fn repo_tags(&self) -> Option<Vec<String>>
    {
        self.repo_tags.clone()
    }

    pub fn repo_tags_name(&self) -> Option<String>
    {
        if let Some(ref tag) = self.repo_tags {
            let names = tag[0].split(":").collect::<Vec<_>>();
            Some(names[0].to_string().replace("\"", ""))
        } else {
            None
        }
    }
}