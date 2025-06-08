use fs_extra::dir::{CopyOptions, copy};
use fs_extra::error::Error as FsError;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config;

#[derive(Clone)]
pub struct File {
    pub path: String,
}

impl File {
    pub fn content(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.path)
    }
}

pub trait FileType {
    const EXTENSION: &'static str;

    fn subdirectory() -> &'static str {
        ""
    }
}

pub fn get_all<T: FileType>(template: &config::Config) -> Vec<File> {
    let location = format!("{}/{}", template.source_dir, T::subdirectory());
    get_files(Path::new(&location), T::EXTENSION)
}

pub fn assets(config: &config::Config) -> Result<u64, FsError> {
    let src_assets = PathBuf::from(format!("{}/{}", config.source_dir, "assets"));
    let dst_assets = PathBuf::from(format!("{}/{}", config.destination_dir, "."));

    copy_files(&src_assets, &dst_assets)
}

fn copy_files(src: &Path, dst: &Path) -> Result<u64, FsError> {
    let options = CopyOptions::new().copy_inside(false).overwrite(true);
    copy(src, dst, &options)
}

fn get_files(dst: &Path, filetype: &str) -> Vec<File> {
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
                    .map(|ext| ext == filetype)
                    .unwrap_or(false)
        })
        .filter_map(|path| path.to_str().map(|p| File { path: p.into() }))
        .collect()
}
