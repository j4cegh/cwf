use std::env;
use std::fs::{self, File, read_to_string};
use std::io::Write;

pub fn dist_css() {
    let dir = env::current_dir().unwrap();
    let src_dir = dir.join("src");

    let mut css_files = Vec::new();
    for entry in fs::read_dir(&src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap() == "css" {
            css_files.push(path);
        }
    }

    for entry in css_files {
        let mut file = File::create(&format!("{}/dist/{}", dir.to_str().unwrap(), entry.file_name().unwrap().to_str().unwrap())).unwrap();
        let file_value = read_to_string(entry).unwrap();
        file.write_all(file_value.as_bytes()).unwrap();
    }
}