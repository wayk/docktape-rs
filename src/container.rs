use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;

pub struct Container{
    pub id: String,
    pub name: String,
    pub image: String,
    pub running: bool
}

impl Container{
    pub fn id(&self) -> String{
        self.id.clone().replace("\"", "")
    }

    pub fn name(&self) -> String{
        self.name.clone().replace("\"", "")
    }

    pub fn image(&self) -> String{
        self.image.clone().replace("\"", "")
    }

    pub fn running(&self) -> bool{
        self.running.clone()
    }
}