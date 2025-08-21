use crate::filemanager::Resource;

#[derive(Debug, Default)]
pub struct Pagination {
    pub per_page: Option<usize>,
}
#[derive(Debug, Default)]
pub struct ThemeConfig {
    pub pagination: Pagination,
}
#[derive(Debug)]
pub struct Config {
    pub source_dir: String,
    pub destination_dir: String,
    pub theme_config: ThemeConfig,
}

impl Config {
    pub fn new(src: String, dst: String) -> Config {
        let theme_config_path = format!("{}/{}.{}", src, "config", "yaml");
        let theme_config_file = Resource {
            path: theme_config_path,
        };
        // I dont wanna have a whole lib like serde to just parse one simple static yaml styled config (theme_config_parser)
        let theme_config = match theme_config_file.contents() {
            Ok(c) => Config::theme_config_parser(c),
            Err(_) => ThemeConfig::default(),
        };

        Config {
            source_dir: src,
            destination_dir: dst,
            theme_config: theme_config,
        }
    }
    fn theme_config_parser(content: String) -> ThemeConfig {
        let mut config = ThemeConfig::default();
        let mut current_section: String = String::new();
        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.ends_with(':') {
                current_section = trimmed.trim_end_matches(':').to_string();
                continue;
            }

            if let Some((key, val)) = trimmed.split_once(':') {
                let key = key.trim();
                let val = val.trim();
                if current_section.as_str() == "pagination" && key == "per_page" {
                    config.pagination.per_page = val.parse().ok();
                }
            }
        }
        config
    }
}
