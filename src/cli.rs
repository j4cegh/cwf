use crate::{ts, web};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc};
use std::thread::sleep;
use std::time::Duration;
use std::{fs, thread};
use std::fs::{File, read_to_string};

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    port: i32,
}

pub fn run_cli(args: &Vec<String>) {
    let option = &args[1];
    let args = args[2..].to_vec();

    match option.as_str() {
        "create-project" | "new" => create_project(args),
        "run" => run_project(args),
        _ => {}
    }
}

fn create_project(option: Vec<String>) {
    println!("Creating project...");

    if option[0].len() == 0 {
        println!("Please provide a project name.");
        return;
    }

    let project_name = option[0].to_string();

    if fs::metadata(&project_name).is_ok() {
        println!("Project already exists.");
        return;
    }

    fs::create_dir(&project_name).unwrap();

    shape_project(&project_name);
}

fn shape_project(project_name: &String) {
    fs::create_dir(format!("{}/src", project_name)).unwrap();
    File::create(&format!("{}/src/{}", &project_name, "index.ts")).unwrap();
    File::create(&format!("{}/src/{}", &project_name, "index.html")).unwrap();
    File::create(&format!("{}/src/{}", &project_name, "index.css")).unwrap();

    let project_json = "\
{
    \"name\": \"".to_owned() + project_name + "\",
    \"port\": 3000
}";
    fs::write(
        &format!("{}/{}", &project_name, "project.json"),
        project_json,
    ).unwrap();

    println!("Project created.");
}

fn run_project(option: Vec<String>) {
    let dir = env::current_dir().unwrap();

    let mut project_string = String::new();
    let mut project_file =
        File::open(&format!("{}/{}", &dir.display(), "project.json")).unwrap();
    project_file.read_to_string(&mut project_string).unwrap();

    let mut index_string = String::new();
    let mut index_file = File::open(&format!("{}/src/{}", &dir.display(), "index.html")).unwrap();
    index_file.read_to_string(&mut index_string).unwrap();

    // convert directory typescript files to javascript
    conv_dir_ts_to_js(&dir);

    dist_css();
    let p: Project = serde_json::from_str(&*project_string).unwrap();
    web::start(index_string, p.port);
    loop {}
}

fn conv_ts_to_js(ts_file: &str) {
    let file = Path::new(ts_file);
    let file_name = file.file_name().unwrap().to_str().unwrap();

    let file_js = ts::convert_ts(&ts_file.to_string());

    let mut file_js_path = PathBuf::new();
    file_js_path.push(file.parent().unwrap().parent().unwrap());
    file_js_path.push("dist");
    file_js_path.push(file_name);
    file_js_path.set_extension("js");

    let mut file_js_file = File::create(file_js_path).unwrap();
    file_js_file.write_all(file_js.as_bytes()).unwrap();
}

fn conv_dir_ts_to_js(dir: &PathBuf) {
    // load ts files to vec
    let mut ts_files = Vec::new();
    let src_dir = dir.join("src");

    for entry in fs::read_dir(&src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap() == "ts" {
            ts_files.push(path);
        }
    }

    // convert the ts files to js files before run
    for entry in ts_files {
        let ts_file_name = entry.to_str().unwrap();
        conv_ts_to_js(ts_file_name);
    }
}

fn dist_css() {
    let dir = env::current_dir().unwrap();
    let src_dir = dir.join("src");
    let dist_dir = dir.parent().unwrap().join("dist");

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