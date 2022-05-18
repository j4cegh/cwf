use crate::{dist, web};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read};
use std::fs::{self, File};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    name: String,
    port: i32,
    page_map: Map<String, Value>,
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
    fs::create_dir(format!(r"{}/src", project_name)).unwrap();
    fs::create_dir(format!(r"{}/dist", project_name)).unwrap();
    fs::create_dir(format!(r"{}/public", project_name)).unwrap();

    File::create(&format!(r"{}/src/{}", &project_name, "index.ts")).unwrap();
    File::create(&format!(r"{}/src/{}", &project_name, "index.html")).unwrap();
    File::create(&format!(r"{}/src/{}", &project_name, "index.css")).unwrap();

    let project_json = "\
{
   \"name\": \"".to_owned() + project_name + "\",
   \"port\": 3000,
   \"pageMap\": {
       \"/\": \"index.html\"
   }
}";
    fs::write(
        &format!(r"{}/{}", &project_name, "project.json"),
        project_json,
    ).unwrap();

    println!("Project created.");
}

fn run_project(_option: Vec<String>) {
    let dir = env::current_dir().unwrap();

    let mut project_string = String::new();
    let mut project_file =
        File::open(&format!(r"{}/{}", &dir.display(), "project.json")).unwrap();
    project_file.read_to_string(&mut project_string).unwrap();

    let p: Project = serde_json::from_str(&*project_string).unwrap();

    // recreate the dist folder so no issues happen
    dist::recreate(dir.join("dist"));

    // dist all the needed files
    dist::dist(dir);

    // start the web server
    web::start(p.port, p.page_map);

    loop {}
}
