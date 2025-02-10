use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use tera::{Context, Tera};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Post {
    pub template_base: Option<String>,
    pub title: String,
    pub slug: String,
    pub date: String,
    pub intro: String,
    pub author: String,
    pub size: String,
    pub text: String,
}

impl Post {
    pub fn new(
        title: String,
        slug: String,
        author: String,
        date: String,
        intro: String,
        text: String,
        template_base: Option<String>,
    ) -> Post {
        let size_count: usize = text.as_bytes().len() / 1024;
        let mut size = format!("{}Kb", size_count);
        if size_count <= 1 {
            size = format!("{}b", text.as_bytes().len());
        }
        Post {
            template_base,
            title,
            slug,
            author,
            date,
            size,
            intro,
            text,
        }
    }
    pub fn create(&self, template: &String, location: &String) -> Result<()> {
        let loc = format!("{}/{}", &location, &self.slug);
        let loc = Path::new(&loc);

        let mut file = File::create(&loc).unwrap();
        let mut html: String = String::new();

        if let Some(base) = &self.template_base {
            let tbase = format!("{} \"{}\" {}", "{%extends", base, "%}");
            html.push_str(&tbase);
        }
        let body = String::from("{% block main %}") + self.text.as_str() + "{% endblock main %}";

        html.push_str(&body);

        let template_pages = format!("{}/{}", template, "/pages/**/*");
        let mut tera = Tera::new(&template_pages).unwrap();
        let _ = tera.add_raw_template(&loc.to_str().unwrap(), html.as_str());
        let mut context = Context::new();
        context.insert("post", &self);
        let output = tera.render(&loc.to_str().unwrap(), &context).unwrap();

        file.write_all(output.as_bytes())?;

        Ok(())
    }
}
