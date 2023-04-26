use std::path::{PathBuf};
use local_ip_address::local_ip;
use path_clean::PathClean;

pub fn get_ip() -> String {
    match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(_) => "<ERROR>".to_string(),
    }
}

pub fn init_path(path_arg: Option<String>, path_default: &str) -> PathBuf {
    let path = match path_arg {
        Some(val) => {
            let path = PathBuf::from(&val);
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join(path).clean()
        },
        None => PathBuf::from(path_default),
    };

    if !path.exists() {
        panic!("Invalid path: {:?}", path);
    }
    path
}
