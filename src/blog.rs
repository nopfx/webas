use crate::config::Config;
use crate::file::File;
use crate::page::Page;
use crate::post::Post;

use std::fs::File as stdFile;
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

#[derive(Debug)]
pub struct Blog<'a> {
    pub config: &'a Config,
    pub pages: Vec<Page>,
    pub posts: Vec<Post>,
}

impl<'a> Blog<'a> {
    pub fn new(page_files: Vec<File>, post_files: Vec<File>, config: &Config) -> Blog {
        // convert to pages
        let pages: Vec<Page> = page_files.iter().map(|pf| pf.clone().into()).collect();
        // convert to posts
        let posts: Vec<Post> = post_files.iter().map(|pf| pf.clone().into()).collect();

        Blog {
            config: &config,
            pages: pages,
            posts: posts,
        }
    }

    pub fn create(&self) {
        // Create posts
        self.create_posts();
        // create pages
        self.create_pages();
    }

    fn create_pages(&self) {
        for page in &self.pages {
            let page_html_path =
                format!("{}/{}/{}", self.config.source_dir, "pages", page.filename);
            let template_pages = format!("{}/{}", self.config.source_dir, "pages/**/*");
            let mut tera =
                Tera::new(template_pages.as_str()).expect("[-] Page: Cannot load twig template");
            tera.add_raw_template(page_html_path.as_str(), page.content.as_str())
                .expect("[-] Page: wrong page template");

            let mut context = Context::new();
            context.insert("posts", &self.posts);

            let mut html = tera
                .render(&page_html_path, &context)
                .expect("[-] Page: wrong page template");

            let path = Path::new(page_html_path.as_str());
            let file_stem: &str = path
                .file_stem()
                .unwrap()
                .to_str()
                .expect("[-] Page: wrong page name");

            let save_location = format!("{}/{}.{}", self.config.destination_dir, file_stem, "html");
            let mut file = stdFile::create(&save_location).unwrap();

            html = self.minify_html(&html);

            file.write_all(html.as_bytes()).unwrap()
        }
        println!("[+] Page: All pages created!");
    }

    fn create_posts(&self) {
        for post in &self.posts {
            let post_slug = post.meta.slug.as_deref().unwrap_or("".into());
            let post_html_path = format!("{}/{}", self.config.destination_dir, post_slug);

            let mut file = stdFile::create(Path::new(&post_html_path))
                .expect("[-] Post: Cannot create a post file");
            let mut html = String::new();

            if let Some(base) = &post.meta.template_base {
                let tbase = format!("{} \"{}\" {}", "{%extends", base, "%}");
                html.push_str(&tbase);
            }

            let body =
                String::from("{% block main %}") + post.html.as_str() + "{% endblock main %}";

            html.push_str(&body);

            let template_pages = format!("{}/{}", self.config.source_dir, "/pages/**/*");
            let mut tera = Tera::new(&template_pages).unwrap();
            let _ = tera.add_raw_template(&post_html_path.to_string(), html.as_str());
            let mut context = Context::new();
            context.insert("post", post);
            let output = tera.render(&post_html_path.to_string(), &context).unwrap();

            let html = &self.minify_html(&output);

            file.write_all(html.as_bytes()).unwrap()
        }
        println!("[+] Post: All posts created!")
    }

    fn minify_html(&self, html: &str) -> String {
        let mut result = String::new();
        let mut inside_preserve_block = false;

        for line in html.lines() {
            let trimmed = line.trim();

            if !inside_preserve_block && (trimmed.contains("<pre") || trimmed.contains("<code")) {
                inside_preserve_block = true;
            }

            if inside_preserve_block {
                result.push_str(line); // Preserve formatting
                result.push('\n');
            } else {
                result.push_str(trimmed); // Minify line
            }

            if inside_preserve_block && (trimmed.contains("</pre>") || trimmed.contains("</code>"))
            {
                inside_preserve_block = false;
            }
        }

        result
    }
}
