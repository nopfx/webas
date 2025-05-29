use crate::file;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde_yaml;

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct PostMeta {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub size: Option<String>,
    pub intro: Option<String>,
    pub template_base: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Post {
    pub html: String,
    pub meta: PostMeta,
}

impl From<file::File> for Post {
    fn from(file: file::File) -> Self {
        let content = file.content().expect("[-] Post: cannot read a post file.");

        let re =
            Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)").expect("[-] Post: problems with regex");

        let captures = re
            .captures(content.as_str())
            .expect("[-] Post: must have a meta fields.");

        let markdown_content = match captures.get(2).map(|m| m.as_str()) {
            Some(e) => e,
            None => "".into(),
        };

        let yaml_content = captures
            .get(1)
            .map(|m| m.as_str())
            .expect("[-] Meta data should be written in yaml format");

        let parser = Parser::new_ext(markdown_content, Options::all());

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let post_meta: PostMeta = serde_yaml::from_str(yaml_content).unwrap_or_default();

        return Post {
            html: html_output,
            meta: post_meta,
        };
    }
}

impl file::FileType for Post {
    const EXTENSION: &'static str = "md";

    fn subdirectory() -> &'static str {
        "posts"
    }
}
