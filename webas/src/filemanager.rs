use crate::config;

use std::fs;
use std::io;
use std::path::Path;

use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct Resource {
    pub path: String,
}

impl Resource {
    pub fn contents(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.path)
    }
}

pub trait FileType {
    const EXT: &'static str;

    fn subdirectory() -> &'static str {
        ""
    }
}

pub fn copy_all(src: &str, dst: &str) -> io::Result<()> {
    let dst = Path::new(&dst);

    if !dst.exists() {
        fs::create_dir_all(&dst).expect("Failed to create destination asset directory!");
    }

    let entries = fs::read_dir(src).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = entry.file_name();
        let dst_asset = dst.join(filename);

        if path.is_dir() {
            let _ = copy_all(path.to_str().unwrap(), dst_asset.to_str().unwrap());
        } else {
            fs::copy(&path, &dst_asset).unwrap();
        }
    }

    Ok(())
}

pub fn list<T: FileType>(config: &config::Config) -> Vec<Resource> {
    let location = format!("{}/{}", config.source_dir, T::subdirectory());
    files_in(Path::new(&location), T::EXT)
}

fn files_in(dst: &Path, ext: &str) -> Vec<Resource> {
    fs::read_dir(dst)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|e| e == ext)
                    .unwrap_or(false)
        })
        .filter_map(|path| path.to_str().map(|p| Resource { path: p.into() }))
        .collect()
}

pub fn save(filepath: String, content: &[u8]) {
    let mut file = File::create(filepath).expect("[-] Page: cannot save a page!");
    file.write_all(content)
        .expect("[-] Page: cannot write to page!");
}
