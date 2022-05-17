use rouille::Response;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::thread;
use mime_guess;
use serde_json::{Map, Value};
use crate::{css, html};
use std::env;
use std::path::{Path, PathBuf};

pub fn start(port: i32, page_map : Map<String, Value>) {
    let start = std::time::Instant::now();
    thread::spawn(move || {
        rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
            let url = request.url();
            let response = match url.as_str() {
                _ => {
                    // if it's a defined page
                    if let Some(page) = page_map.get(url.as_str()) {
                        let page = page.as_str().unwrap();
                        let dir = env::current_dir().unwrap();

                        // load the html file from the provided path
                        println!("Loading page: {}", page);
                        let page = html::load_page(page);
                        let page_js = html::replace_ts(&page);

                        Response::html(page_js)
                    }
                    // if it's not, try to send a file from dist
                    else {
                        let file = File::open(format!("dist/{}", url.as_str()));
                        match file {
                            Ok(file) => {
                                if url.as_str().ends_with(".css") {
                                    css::dist_css();
                                }
                                let content_type = mime_guess::from_path(format!("dist/{}", url.as_str().replace("/", ""))).first_or_octet_stream();
                                Response::from_file(content_type.to_string(), file)
                            },
                            // if it's not in dist, it's not found within the searched scope.
                            Err(_) => Response::text("Not found.").with_status_code(404),
                        }
                    }
                }
            };
            response
        });
    });
    let duration = start.elapsed().as_micros();
    println!(
        "🚀 Running on port {}. Took {} microseconds!",
        port, duration
    );
    drop(start);
}
