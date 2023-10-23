use std::env;

pub fn get_executable_directory() -> String {
    let exe_path: std::path::PathBuf =
        env::current_exe().expect("Failed to retrieve the path of the executable");
    let exe_dir: &std::path::Path = exe_path
        .parent()
        .expect("Failed to get parent directory of the executable")
        .parent()
        .expect("Failed to get parent directory of the executable")
        .parent()
        .expect("Failed to get parent directory of the executable");
    let dir_str: &str = exe_dir
        .to_str()
        .expect("Failed to convert directory path to string");

    let mut dir_string: String = dir_str.to_string();
    dir_string.push_str("/assets/");
    dir_string
}
