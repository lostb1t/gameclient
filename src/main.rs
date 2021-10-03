#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::process::Command;

pub const APPLICATION_JSON: &str = "application/json";

#[get("/setprimarydisplay/{id}")]
pub async fn setprimarydisplay(path: web::Path<String>) -> HttpResponse {
    let current_dir = std::env::current_dir().unwrap();
    let display_number = path.0.as_str();
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(current_dir)
            .arg(format!("nircmd.exe setprimarydisplay  {}", display_number))
            .output()
            .expect("failed to execute process");
        String::from("succes")
    } else {
        String::from("Only supported on windows")
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(output)
    // HttpResponse::NoContent()
    //     .content_type(APPLICATION_JSON)
    //     .await
    //     .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(setprimarydisplay)
    })
    .workers(1)
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
