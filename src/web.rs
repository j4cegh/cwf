use rouille::Response;
use std::fs::File;
use std::thread;
use mime_guess;
use serde_json::{Map, Value};
use crate::{css, html};

pub fn get_file_name(full_path: &str) -> String {
    let mut path_vec = full_path.split("/").collect::<Vec<&str>>();
    let file_name = path_vec.pop().unwrap();
    file_name.to_string()
}

// tries to get a file from the public folder; makes a "Not found." response if not found.
fn get_public(url: String) -> Response {
    let file_name = get_file_name(&url);

    let mut path_vec = url.split("/static").collect::<Vec<&str>>();
    let path = path_vec.pop().unwrap();

    let file = File::open(format!(r"public\{}", path));

    let response = match file {
        Ok(file) => {
            let content_type = mime_guess::from_path(format!(r"public\{}", file_name)).first_or_octet_stream();
            Response::from_file(content_type.to_string(), file)
        }
        Err(_) => {
            Response::text("Not found.").with_status_code(404)
        }
    };
    response
}

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

                        // load the html file from the provided path
                        println!("Loading page: {}", page);
                        let page = html::load_page(page);

                        Response::html(html::replace_ts(&page))
                    }
                    // if it's not, try to send a file from dist
                    else {
                        // static stuff
                        if url.starts_with("/static/") {
                            return get_public(url.to_string());
                        }

                        let file = File::open(format!("dist/{}", url.as_str()));
                        match file {
                            Ok(file) => {
                                if url.as_str().ends_with(".css") {
                                    css::dist_css();
                                }
                                let content_type = mime_guess::from_path(format!("dist/{}", get_file_name(&url))).first_or_octet_stream();
                                Response::from_file(content_type.to_string(), file)
                            },
                            // if it's not from dist, try to send from public.
                            Err(_) => Response::text("Not found.").with_status_code(404)
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
