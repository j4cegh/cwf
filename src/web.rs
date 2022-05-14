use rouille::Request;
use rouille::Response;
use std::fmt::format;
use std::fs::File;
use std::thread;

pub fn start(index: String, port: i32) {
    let start = std::time::Instant::now();
    let rouille_thread = thread::spawn(move || {
        rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
            let url = request.url();
            let response = match url.as_str() {
                "/" => Response::html(format!("{}", index.replace(".ts", ".js"))),
                _ => {
                    let file = File::open(format!("{}/{}", "dist", url.as_str().replace("/", "")));

                    match file {
                        Ok(file) => Response::from_file("application/javascript", file),
                        Err(_) => Response::text("Not found.").with_status_code(404),
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
