mod argc;
mod blog;
mod config;
mod filemanager;
mod page;
mod post;

use std::path::Path;

fn main() {
    let config = config::Config::new(
        argc::get::<String>("src").unwrap_or_default(),
        argc::get::<String>("dst").unwrap_or_default(),
    );

    if !Path::new(&config.source_dir).exists() || !Path::new(&config.destination_dir).exists() {
        panic!("\n\nProbably wrong arguments,\nUsage:\n\n\t--src <ource files folder>\n\t--dst <compiled web files folder>\n\nThese directories required!");
    }

    filemanager::copy_all(
        &format!("{}/{}", &config.source_dir, "assets"),
        &format!("{}/{}", &config.destination_dir, "assets"),
    )
    .expect("Cannot copy assets");

    let posts = filemanager::list::<post::Post>(&config);
    let pages = filemanager::list::<page::Page>(&config);

    let blog = blog::Blog::new(pages, posts, &config);
    blog.create();
}
