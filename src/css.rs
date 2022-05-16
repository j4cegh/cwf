use std::env;
use std::fs::{self, create_dir_all, File, read_to_string};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::{dist, ts};

pub fn dist_css() {
    let dir = env::current_dir().unwrap();

    for entry in WalkDir::new(dir.join("src").to_str().unwrap()) {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name == "src" {
            continue;
        }

        if path.is_dir() {
            let f_path_dist = format!(r"{}\dist\{}", dir.to_str().unwrap(), file_name);
            println!("Log[NF]: {}", f_path_dist);
            if !Path::new(&f_path_dist).exists() {
                create_dir_all(&f_path_dist).unwrap();
            }
        }
        else if path.is_file() {
            let file_path = path.to_str().unwrap().split("src").collect::<Vec<&str>>()[1];
            let f_path_dist = format!(r"{}\dist\{}", dir.to_str().unwrap(), file_path);

            if file_name.ends_with(".css") {
                File::create(Path::new(&f_path_dist)).expect("Couldn't create dist file.");

                let mut file = File::open(path).expect("Couldn't open file.");
                let mut content = String::new();
                file.read_to_string(&mut content).expect("Couldn't read file.");
                let mut file = File::create(Path::new(&f_path_dist)).expect("Couldn't create dist file.");
                file.write_all(content.as_bytes()).expect("Couldn't write file.");
            }
        }
    }
}