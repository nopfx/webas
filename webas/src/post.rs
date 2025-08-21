use crate::filemanager;

use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

htnl::context! {
    struct PostMeta {
        title: Option<String>,
        slug: Option<String>,
        author: Option<String>,
        date: Option<String>,
        size: Option<String>,
        intro: Option<String>,
        base_resource: Option<String>,
    }
}

htnl::context! {
    struct Post {
        html: String,
        meta: PostMeta,
    }
}

impl filemanager::FileType for Post {
    const EXT: &'static str = "md";
    fn subdirectory() -> &'static str {
        "posts"
    }
}

impl From<filemanager::Resource> for Post {
    fn from(file: filemanager::Resource) -> Self {
        let content = file.contents().expect("[-] Post: cant read a post file.");

        let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)")
            .expect("[-] Post: cannot compile regex");

        let captures = re
            .captures(content.as_str())
            .expect("[-] Post: meta fields required in post file!");

        let meta_string = match captures.get(1).map(|m| m.as_str()) {
            Some(e) => e,
            None => "",
        };
        let mut postmeta = PostMeta::default();
        for meta_attribute in meta_string.lines() {
            if let Some((key, val)) = meta_attribute.split_once(':') {
                match key.trim() {
                    "title" => postmeta.title = Some(String::from(val)),
                    "slug" => postmeta.slug = Some(String::from(val)),
                    "author" => postmeta.author = Some(String::from(val)),
                    "date" => postmeta.date = Some(String::from(val)),
                    "size" => postmeta.size = Some(String::from(val)),
                    "intro" => postmeta.intro = Some(String::from(val)),
                    "base_resource" => postmeta.base_resource = Some(String::from(val)),
                    _ => {}
                }
            }
        }

        let markdown_content = match captures.get(2).map(|m| m.as_str()) {
            Some(e) => e,
            None => "",
        };
        let parser = Parser::new_ext(markdown_content, Options::all());
        let mut mktohtml = String::new();
        html::push_html(&mut mktohtml, parser);

        Post {
            html: mktohtml,
            meta: postmeta,
        }
    }
}
