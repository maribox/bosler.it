use std::path::{PathBuf};
use path_clean::PathClean;


pub fn init_path(path_arg: Option<String>, path_default: &str) -> PathBuf {
    let path = match path_arg {
        Some(val) => {
            let path = PathBuf::from(&val);
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join(path).clean()
        },
        None => PathBuf::from(path_default),
    };

    if !path.exists() {
        panic!("Invalid path provided: {:?}", path);
    }
    path
}
