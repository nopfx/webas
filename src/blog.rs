use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use tera::{Context, Error, Tera};

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
        let loc = format!("{}/{}", &location, &self.slug);

        let loc = Path::new(&loc);
        let mut file = File::create(&loc).unwrap();

        let mut html = String::from("{% extends \"parts/base.twig\" %}");
        //let title =
        //    String::from("{% block title %}") + self.title.as_str() + "{% endblock title %}";
        let body = String::from("{% block main %}") + self.text.as_str() + "{% endblock main %}";

        //html.push_str(&title);
        html.push_str(&body);

        let mut tera = Tera::new("./tests/template/pages/**/*").unwrap();
        let _ = tera.add_raw_template(&loc.to_str().unwrap(), html.as_str());

        let output = tera
            .render(&loc.to_str().unwrap(), &Context::new())
            .unwrap();

        file.write_all(output.as_bytes())?;

        Ok(())
    }
}
