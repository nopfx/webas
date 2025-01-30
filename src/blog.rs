use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub date: String,
    pub intro: String,
    pub text: String,
}

impl Post {
    pub fn new(title: String, slug: String, date: String, intro: String, text: String) -> Post {
        Post {
            title,
            slug,
            date,
            intro,
            text,
        }
    }
    pub fn create(&self, location: String) -> Result<()> {
        let loc = Path::new(&location).join(&self.slug);
        let mut file = File::create(loc)?;
        file.write_all(self.text.as_bytes())?;
        Ok(())
    }
}
