use std::fs;

#[derive(Debug)]
pub struct File {
    pub location: String,
}

impl File {
    pub fn new(location: &str) -> File {
        File {
            location: location.into(),
        }
    }
    pub fn content(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(&self.location)
    }
}
