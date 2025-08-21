use crate::config::Config;
use crate::filemanager::{save, Resource};
use crate::page::Page;
use crate::post::Post;

use htnl::{builder::Builder, contextable::Contextable, render};

#[derive(Debug)]
pub struct Blog<'a> {
    pub config: &'a Config,
    pub pages: Vec<Page>,
    pub posts: Vec<Post>,
}

htnl::context! {
    struct PostsContext {
        posts: Vec<Post>
    }
}

impl<'a> Blog<'a> {
    pub fn new(pages_list: Vec<Resource>, posts_list: Vec<Resource>, config: &Config) -> Blog {
        let pages: Vec<Page> = pages_list.iter().map(|f| f.clone().into()).collect();
        let posts: Vec<Post> = posts_list.iter().map(|f| f.clone().into()).collect();

        Blog {
            config: &config,
            pages,
            posts,
        }
    }

    pub fn create(&self) {
        let posts = PostsContext {
            posts: self.posts.clone(),
        };
        self.create_pages(&posts);
        self.create_posts(&posts);
    }

    fn create_pages(&self, posts: &PostsContext) {
        for page in &self.pages {
            let context = posts.flatten();

            let htdl = Builder {
                context,
                content: page.content.to_string(),
            };

            let compiled = htdl.build();

            let filename_tosave = page.filename.replace(".htnl", ".html");
            let fullpath = format!("{}/{}", self.config.destination_dir, filename_tosave);

            save(fullpath, &compiled.as_bytes());
        }
        println!("[+] All pages created!");
    }

    fn create_posts(&self, posts: &PostsContext) {
        for post in &posts.posts {
            let slug = &post.meta.slug.as_deref().unwrap_or_default();
            let filepath = format!("{}/{}", self.config.destination_dir.trim(), slug.trim());

            if let Some(base) = &post.meta.base_resource {
                let templatepath = format!("{}/{}", self.config.source_dir.trim(), base.trim());
                let context = post.flatten();

                let compiled = render(templatepath.as_str(), context);

                save(filepath, &compiled.as_bytes());
            }
        }
        println!("[+] All posts created");
    }
}
