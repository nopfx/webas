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
    let template_location = cli::get::<String>("template".into()).unwrap_or("".into());
    let web_location = cli::get::<String>("web".into()).unwrap_or("./".into());

    if Path::new(&web_location).exists() {
        fs::remove_dir_all(&web_location).unwrap();
    }

    let _ = fs::create_dir(&web_location);
    let options = CopyOptions::new().copy_inside(true);
    let src_location = format!("{}/{}", &template_location, "/assets");
    let src_folder = Path::new(&src_location);
    let dst_location = format!("{}/{}", &web_location, "/assets");
    let dst_folder = Path::new(&dst_location);
    copy(src_folder, dst_folder, &options).unwrap();

    let posts: Vec<Post> =
        loaders::get_files_by_type(format!("{}/{}", &template_location, "/posts"), "md".into())
            .into_iter()
            .map(|f| Markdown::new(f))
            .map(|m| m.to_post().unwrap())
            .collect();

    let templates: Vec<Twig> = loaders::get_files_by_type(
        format!("{}/{}", &template_location, "/pages"),
        "twig".into(),
    )
    .into_iter()
    .map(|f| Twig::new(f, &template_location))
    .collect();

    for template in templates {
        let mut context = Context::new();
        context.insert("posts", &posts);
        let path = Path::new(template.file.location.as_str());
        let file_stem: &str = path.file_stem().unwrap().to_str().unwrap();

        let file_location = format!("{}/{}.{}", &web_location, file_stem, "html");
        let mut file = File::create(&file_location).unwrap();
        let _ = file.write_all(template.to_html(context).unwrap().as_bytes());
    }

    for post in &posts {
        let _ = post.create(&template_location, &web_location);
    }
}
