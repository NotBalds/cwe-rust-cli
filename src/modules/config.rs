use crate::base::filesystem;
use directories::ProjectDirs;
use std::path::PathBuf;

pub const VERSION: &str = "0.3.0-dev-0";
pub const DEV_MODE: bool = true;
pub const SYS_BITS: u32 = 2048;
pub const BASE_BITS: u32 = 4096;
pub const BLOCK_BITS: u32 = 1048576;

pub const SUPPORTED_TYPES: [&str; 2] = ["Text", "Image"];

pub fn path() -> PathBuf {
    let result_path = ProjectDirs::from("su", "bald", "cwe-client")
        .unwrap()
        .data_local_dir()
        .to_path_buf();
    match DEV_MODE {
        true => result_path.join("dev"),
        false => result_path,
    }
}

pub fn default_url() -> String {
    String::from("http://127.0.0.1:1337/")
}

pub fn supported_types() -> Vec<String> {
    SUPPORTED_TYPES.iter().map(|x| x.to_string()).collect()
}

pub fn url(path: &str) -> String {
    let base = filesystem::cat(&filesystem::new_path("server"));

    base + &path.to_string()
}
