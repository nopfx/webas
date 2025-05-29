use crate::file;

use std::path::Path;

#[derive(Debug)]
pub struct Page {
    pub filename: String,
    pub content: String,
}

impl From<file::File> for Page {
    fn from(file: file::File) -> Self {
        let file_path = Path::new(file.path.as_str());
        let filename = file_path.file_name().expect("[-] Page: Invalid file name.");
        let content = match file.content() {
            Ok(c) => c,
            Err(_) => "".into(),
        };

        Page {
            filename: filename.to_string_lossy().into(),
            content: content,
        }
    }
}

impl Page {}

impl file::FileType for Page {
    const EXTENSION: &'static str = "twig";

    fn subdirectory() -> &'static str {
        "pages"
    }
}
