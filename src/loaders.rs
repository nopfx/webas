use crate::readers::File;
use std::fs;

pub fn get_files_by_type(location: String, file_type: String) -> Vec<File> {
    let mut md_files: Vec<File> = vec![];
    if let Ok(entries) = fs::read_dir(location) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file()
                    && path
                        .extension()
                        .map(|ext| ext.to_str().unwrap_or("".into()) == file_type.as_str())
                        .unwrap_or(false)
                {
                    match path.to_str() {
                        Some(file) => md_files.push(File::new(file)),
                        None => {}
                    }
                }
            }
        }
    }
    md_files
}
