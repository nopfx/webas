use regex::Regex;
use std::collections::HashMap;

pub fn clean_html(input: &str) -> String {
    let pre_regex = Regex::new(r"(?s)<pre>.*?</pre>").unwrap();
    let newline_regex = Regex::new(r"\r?\n").unwrap();
    let spaces_between_tags = Regex::new(r">\s+<").unwrap();

    // Step 1: Store <pre> content and replace with placeholders
    let mut pre_blocks: HashMap<String, String> = HashMap::new();
    let mut counter = 0;
    let content_with_placeholders = pre_regex
        .replace_all(input, |caps: &regex::Captures| {
            let placeholder = format!("__PRE_BLOCK_{}__", counter);
            pre_blocks.insert(placeholder.clone(), caps[0].to_string());
            counter += 1;
            placeholder
        })
        .to_string();

    // Step 2: Remove newlines globally (except <pre> placeholders)
    let cleaned_content = newline_regex
        .replace_all(&content_with_placeholders, "")
        .to_string();

    // Step 3: Remove spaces between HTML tags (except inside <pre>)
    let minified_content = spaces_between_tags
        .replace_all(&cleaned_content, "><")
        .to_string();

    // Step 4: Restore <pre> blocks
    let final_output = pre_blocks
        .iter()
        .fold(minified_content, |acc, (placeholder, pre_content)| {
            acc.replace(placeholder, pre_content)
        });

    final_output
}
