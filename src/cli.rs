use crate::{ts, web};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::thread::sleep;
use std::time::Duration;
use std::{fs, thread};

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    port: i32,
}

pub fn run_cli(args: &Vec<String>) {
    let option = &args[1];
    let args = args[2..].to_vec();

    match option.as_str() {
        "create-project" => create_project(args),
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
    fs::File::create(&format!("{}/src/{}", &project_name, "index.ts")).unwrap();
    fs::File::create(&format!("{}/src/{}", &project_name, "index.html")).unwrap();
    fs::File::create(&format!("{}/src/{}", &project_name, "index.css")).unwrap();

    let project = Project {
        name: project_name.to_string(),
        port: 3000,
    };
    let project_json = serde_json::to_string(&project).unwrap();
    fs::write(
        &format!("{}/{}", &project_name, "project.json"),
        project_json,
    )
    .unwrap();

    println!("Project created.");
}

fn run_project(option: Vec<String>) {
    let dir = env::current_dir().unwrap();

    let mut project_string = String::new();
    let mut project_file =
        fs::File::open(&format!("{}/{}", &dir.display(), "project.json")).unwrap();
    project_file.read_to_string(&mut project_string).unwrap();

    let mut index_string = String::new();
    let mut index_file = fs::File::open(&format!("{}/{}", &dir.display(), "index.html")).unwrap();
    index_file.read_to_string(&mut index_string).unwrap();

    // convert directory typescript files to javascript
    conv_dir_ts_to_js(&dir);

    let p: Project = serde_json::from_str(&*project_string).unwrap();
    web::start(index_string, p.port);
    loop {}
}

fn conv_ts_to_js(ts_file: &str) {
    let js_code = ts::convert_ts(ts_file);
    let mut js_file_name = String::new();
    js_file_name = ts_file.to_string().replace(".ts", ".js");

    let dist_dir = "dist";
    if !PathBuf::from(dist_dir).exists() {
        fs::create_dir(dist_dir).unwrap();
    }

    let mut file = fs::File::create(&format!("{}/{}", dist_dir, &js_file_name)).unwrap();
    file.write_all(js_code.as_bytes()).unwrap();
}

fn conv_dir_ts_to_js(dir: &PathBuf) {
    // load ts files to vec
    let mut ts_files = Vec::new();
    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap() == "ts" {
            ts_files.push(path);
        }
    }

    // convert the ts files to js files before run
    for entry in ts_files {
        let mut ts_string = String::new();
        let mut ts_file = fs::File::open(&entry).unwrap();
        ts_file.read_to_string(&mut ts_string).unwrap();
        let ts_file_name = entry.file_name().unwrap().to_str().unwrap();

        conv_ts_to_js(ts_file_name);
    }
}
