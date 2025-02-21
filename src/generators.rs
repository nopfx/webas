use crate::blog::Post;
use crate::parsers::clean_html;
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
    pub template: String,
}

#[derive(serde::Deserialize, Default)]
struct Meta {
    title: Option<String>,
    slug: Option<String>,
    author: Option<String>,
    date: Option<String>,
    intro: Option<String>,
    template_base: Option<String>,
}

impl Twig {
    pub fn new(file: File, template: &String) -> Twig {
        Twig {
            file,
            template: String::from(template),
        }
    }
    pub fn to_html(&self, data: Context) -> Result<String, Error> {
        let template_pages = format!("{}/{}", self.template, "/pages/**/*");
        let tera = Tera::new(template_pages.as_str())?;
        let path: &str = Path::new(&self.file.location)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let html = tera.render(path, &data)?;

        let clean_html = clean_html(html.as_str());

        Ok(clean_html)
    }
}

impl Markdown {
    pub fn new(file: File) -> Markdown {
        Markdown { file }
    }

    pub fn to_post(&self) -> Result<Post, Error> {
        let content = self.file.content().unwrap_or("".into());

        let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)").expect("Cant make regex");
        let captures = re
            .captures(content.as_str())
            .expect("Meta required for Markdowns");

        let markdown_content = captures
            .get(2)
            .map(|m| m.as_str())
            .expect("Markdown is required");
        let yaml_content = captures
            .get(1)
            .map(|m| m.as_str())
            .expect("Metadata is required for Markdown post");

        let parser = Parser::new_ext(markdown_content, Options::all());

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let meta: Meta = serde_yaml::from_str(yaml_content).unwrap_or_default();

        let post = Post::new(
            meta.title.unwrap_or("no-title-in-meta".into()),
            meta.slug.unwrap_or("no-slug-in-meta.html".into()),
            meta.author.unwrap_or("".into()),
            meta.date.unwrap_or("".into()),
            meta.intro.unwrap_or("".into()),
            html_output,
            meta.template_base,
        );

        Ok(post)
    }
}
