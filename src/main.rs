#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::process::Command;

pub const APPLICATION_JSON: &str = "application/json";

#[get("/setprimarydisplay/{id}")]
pub async fn setprimarydisplay(path: web::Path<String>) -> HttpResponse {
    // if cfg!(not(target_os = "windows")) {
    //     return HttpResponse::Ok()
    //     .content_type(APPLICATION_JSON)
    //     .json("Only supported on windows")
    // }

    let current_dir = std::env::current_dir().unwrap();
    let display_number = path.0.as_str();

    log::info!("current dir: {:?}", current_dir);

    let output = Command::new("cmd")
        .current_dir(&current_dir)
        //.arg(format!("./nircmd.exe setprimarydisplay  {}", display_number))
        //.arg("-c")
        //.arg("echo hello")
        //.arg(format!("./test.sh setprimarydisplay  {}", display_number))
        .args(&["/C", format!("nircmd.exe setprimarydisplay  {}", display_number).as_str()])
        .output()
        .expect("failed to execute process");

    match output.status.success() {
        true => log::info!("stdout: {}", String::from_utf8_lossy(&output.stdout)),
        false => log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr)),
    }

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json("succes")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(setprimarydisplay)
    })
    .workers(2)
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
