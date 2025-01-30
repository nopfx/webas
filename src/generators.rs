use crate::blog::Post;
use crate::readers::File;

use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::path::Path;
use tera::{Context, Error, Tera};

#[derive(Debug)]
pub struct Markdown {
    pub file: File,
}

#[derive(Debug)]
pub struct Twig {
    pub file: File,
}

#[derive(serde::Deserialize)]
struct Meta {
    title: String,
    slug: String,
    date: String,
    intro: String,
}

impl Twig {
    pub fn new(file: File) -> Twig {
        Twig { file }
    }
    pub fn to_html(&self, data: Context) -> Result<String, Error> {
        let tera = Tera::new("./tests/template/pages/**/*")?;
        let path: &str = Path::new(&self.file.location)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let html = tera.render(path, &data)?;
        Ok(html)
    }
}

impl Markdown {
    pub fn new(file: File) -> Markdown {
        Markdown { file }
    }

    pub fn to_post(&self) -> Result<Post, serde_yaml::Error> {
        let content = self.file.content().unwrap_or("".into());

        let re = Regex::new(r"(?s)---\n(.*?)\n---\n(.*)").unwrap();
        let captures = re
            .captures(content.as_str())
            .ok_or("Invalid markdown file format")
            .unwrap();
        let yaml_content = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        let markdown_content = captures.get(2).map(|m| m.as_str()).unwrap_or("");
        let parser = Parser::new_ext(markdown_content, Options::all());

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let meta: Meta = serde_yaml::from_str(yaml_content)?;
        let post = Post::new(meta.title, meta.slug, meta.date, meta.intro, html_output);

        Ok(post)
    }
}
