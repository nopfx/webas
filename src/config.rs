use crate::file::File;
use serde_yaml;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Pagination {
    pub per_page: Option<usize>,
    pub paginate: Option<String>,
}
#[derive(Debug, Default, serde::Deserialize)]
pub struct TConfig {
    pub pagination: Pagination,
}
#[derive(Debug)]
pub struct Config {
    pub source_dir: String,
    pub destination_dir: String,
    pub t_config: TConfig,
}

impl Config {
    pub fn new(src: String, dst: String) -> Config {
        let t_config_path = format!("{}/{}.{}", src, "config", "yaml");
        let t_config_file = File {
            path: t_config_path,
        };

        let t_config: TConfig = match t_config_file.content() {
            Ok(content) => serde_yaml::from_str(&content).unwrap_or_default(),
            Err(_) => TConfig::default(),
        };

        println!("[*] Config generated!");

        Config {
            source_dir: src,
            destination_dir: dst,
            t_config: t_config,
        }
    }
}
