use crate::filemanager;

use std::path::Path;

#[derive(Debug)]
pub struct Page {
    pub filename: String,
    pub content: String,
}

impl filemanager::FileType for Page {
    const EXT: &'static str = "htnl";

    fn subdirectory() -> &'static str {
        "pages"
    }
}

impl From<filemanager::Resource> for Page {
    fn from(file: filemanager::Resource) -> Self {
        let file_path = Path::new(file.path.as_str());
        let filename = file_path.file_name().expect("[-] Page: invalid filename");
        let content = file.contents().unwrap_or_default();

        Page {
            filename: filename.to_string_lossy().into(),
            content,
        }
    }
}
