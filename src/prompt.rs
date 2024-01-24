use std::env;

pub fn get_prompt() -> String {
    let username = whoami::username();
    let hostname = whoami::hostname();
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.to_string_lossy();

    if let Some(home_dir) = dirs::home_dir() {
        if current_dir.starts_with(&home_dir) {
            let rel_path = current_dir_str.replacen(&*home_dir.to_string_lossy(), "~", 1);
            return format!("{} on {} {}\n➜ ", username, hostname, rel_path);
        }
    }

    format!("{} on {} {} \n➜ ", username, hostname, current_dir_str)
}

