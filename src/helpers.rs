use std::{env, fs};

#[inline]
pub fn is_in_project_dir() -> bool {
    let current_dir = env::current_dir().expect("Failed to get the current directory path");

    fs::exists(current_dir.join("gornilo.toml"))
        .expect("Failed to check if the command was called from the project root")
}
