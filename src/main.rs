mod blog;
mod cli;
mod generators;
mod loaders;
mod readers;

use blog::Post;
use generators::Markdown;
use generators::Twig;

use fs_extra::dir::{copy, CopyOptions};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::Context;

fn main() {
    if let Some(posts_location) = cli::get::<String>("posts".into()) {
        if let Some(pages_location) = cli::get::<String>("pages".into()) {
            if Path::new("./tests/web").exists() {
                fs::remove_dir_all("./tests/web").unwrap();
            }
            fs::create_dir("./tests/web");
            let options = CopyOptions::new().copy_inside(true);
            let src_folder = Path::new("./tests/template/assets");
            let dst_folder = Path::new("./tests/web/assets");
            copy(src_folder, dst_folder, &options).unwrap();

            let posts: Vec<Post> = loaders::get_files_by_type(posts_location, "md".into())
                .into_iter()
                .map(|f| Markdown::new(f))
                .map(|m| m.to_post().unwrap())
                .collect();

            let templates: Vec<Twig> = loaders::get_files_by_type(pages_location, "twig".into())
                .into_iter()
                .map(|f| Twig::new(f))
                .collect();

            for template in templates {
                let mut context = Context::new();
                context.insert("posts", &posts);
                let path = Path::new(template.file.location.as_str());
                let file_stem: &str = path.file_stem().unwrap().to_str().unwrap();

                let mut file =
                    File::create(String::from("./tests/web/") + file_stem + ".html").unwrap();
                file.write_all(template.to_html(context).unwrap().as_bytes());

                for post in &posts {
                    post.create("tests/web/".into());
                }
            }
        }
    }
}
