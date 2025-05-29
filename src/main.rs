mod argc;
mod blog;
mod config;
mod file;
mod page;
mod post;

use std::error::Error;
use std::path::Path;

fn panic() {
    panic!(
        "Usage:\n\nwebas --src <TEMPLATE> --dst <HTML_DESTINATION>\nDirectories must exists!\n\n"
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config {
        source_dir: argc::get::<String>("src").unwrap_or("".into()),
        destination_dir: argc::get::<String>("dst").unwrap_or("".into()),
    };

    if config.source_dir.len() <= 0
        || config.destination_dir.len() <= 0
        || !Path::new(&config.source_dir).exists()
        || !Path::new(&config.destination_dir).exists()
    {
        panic();
    }

    match file::assets(&config) {
        Ok(_) => println!("[+] Assets: copied successfully!"),
        Err(e) => return Err(format!("[-] assets: Something is wrong: {} ", e).into()),
    }

    let posts = file::get_all::<post::Post>(&config);
    println!("[+] Posts: found {} posts", posts.len());

    let pages = file::get_all::<page::Page>(&config);
    println!("[+] Pages: found {} pages", pages.len());

    let blog = blog::Blog::new(pages, posts, &config);
    println!("[+] Blog: Initialized!");

    blog.create();

    Ok(())
}
