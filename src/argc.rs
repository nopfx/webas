use std::env;

pub fn get<T: std::str::FromStr>(name: &str) -> Option<T> {
    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() {
        if args[i] == format!("--{}", name) {
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                return args[i + 1].parse().ok();
            }
        }
    }
    None
}
