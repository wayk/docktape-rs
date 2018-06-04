use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;

pub struct Network{
    pub id: String,
    pub name: String
}

impl Network{
    pub fn id(&self) -> String{
        self.id.clone().replace("\"", "")
    }

    pub fn name(&self) -> String{
        self.name.clone().replace("\"", "")
    }
}