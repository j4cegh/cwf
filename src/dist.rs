use std::fs::{create_dir, create_dir_all, File, remove_dir_all};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::{ts, css};

pub fn change_ext(old_filename: &str, new_ext: &str) -> String {
    let mut new_filename = old_filename.to_string();
    let dot_pos = new_filename.rfind('.').unwrap_or(new_filename.len());
    new_filename.truncate(dot_pos);
    new_filename.push_str(new_ext);
    new_filename
}

pub fn recreate(dist_dir : PathBuf) {
    let path = dist_dir.as_path().to_str().unwrap();

    remove_dir_all(path).unwrap();
    create_dir(path).unwrap();
}

pub fn dist(dir: PathBuf) {
    for entry in WalkDir::new(dir.join("src").to_str().unwrap()) {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name == "src" {
            continue;
        }

        if path.is_file() {
            if !file_name.ends_with(".ts") && !file_name.ends_with(".css") {
                continue;
            }

            let file_path = path.to_str().unwrap().split("src").collect::<Vec<&str>>()[1];
            let f_path_dist = format!(r"{}\dist\{}", dir.to_str().unwrap(), file_path);
            let path_without_file = f_path_dist.split(file_name).collect::<Vec<&str>>()[0];

            if !Path::new(&path_without_file).exists() {
                create_dir_all(&path_without_file).unwrap();
            }
            if file_name.ends_with(".ts") {
                let f_path_dist = change_ext(&f_path_dist, ".js");

                File::create(Path::new(&f_path_dist)).expect("Couldn't create dist file.");

                let mut file = File::open(path).expect("Couldn't open file.");
                let mut content = String::new();
                file.read_to_string(&mut content).expect("Couldn't read file.");
                let content = ts::convert_ts(path.to_str().unwrap());
                let mut file = File::create(Path::new(&f_path_dist)).expect("Couldn't create dist file.");
                file.write_all(content.as_bytes()).expect("Couldn't write file.");
            }
            else if file_name.ends_with(".css") {
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