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
    pub text: String,
}

impl Post {
    pub fn new(
        title: String,
        slug: String,
        date: String,
        intro: String,
        text: String,
        template_base: Option<String>,
    ) -> Post {
        Post {
            template_base,
            title,
            slug,
            date,
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
        //let title =
        //    String::from("{% block title %}") + self.title.as_str() + "{% endblock title %}";
        //let date = String::from("{% block date %}") + self.date.as_str() + "{% endblock date %}";
        //let slug = String::from("{% block slug %}") + self.slug.as_str() + "{% endblock slug %}";
        //let intro =
        //    String::from("{% block intro %}") + self.intro.as_str() + "{% endblock intro %}";
        let body = String::from("{% block main %}") + self.text.as_str() + "{% endblock main %}";

        //html.push_str(&title);
        //html.push_str(&date);
        //html.push_str(&slug);
        //html.push_str(&intro);
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
