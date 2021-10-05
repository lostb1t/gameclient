#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::process::Command;

pub const APPLICATION_JSON: &str = "application/json";

// #[get("/setprimarydisplay/{id}")]
// pub async fn setprimarydisplay(path: web::Path<String>) -> HttpResponse {
//     // if cfg!(not(target_os = "windows")) {
//     //     return HttpResponse::Ok()
//     //     .content_type(APPLICATION_JSON)
//     //     .json("Only supported on windows")
//     // }

//     let current_dir = std::env::current_dir().unwrap();
//     let display_number = path.0.as_str();

//     log::info!("current dir: {:?}", current_dir);

//     let output = Command::new("cmd")
//         .current_dir(&current_dir)
//         //.arg(format!("./nircmd.exe setprimarydisplay  {}", display_number))
//         //.arg("-c")
//         //.arg("echo hello")
//         //.arg(format!("./test.sh setprimarydisplay  {}", display_number))
//         .args(&[
//             "/C",
//             format!("nircmd.exe setprimarydisplay  {}", display_number).as_str(),
//         ])
//         .output()
//         .expect("failed to execute process");

//     match output.status.success() {
//         true => log::info!("stdout: {}", String::from_utf8_lossy(&output.stdout)),
//         false => log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr)),
//     }

//     HttpResponse::Ok()
//         .content_type(APPLICATION_JSON)
//         .json("succes")
// }


#[get("/setmode/{name}")]
pub async fn setmode(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    // if cfg!(not(target_os = "windows")) {
    //     return HttpResponse::Ok()
    //     .content_type(APPLICATION_JSON)
    //     .json("Only supported on windows")
    // }
    // let config = Config {
    //     displays
    // }
    let current_dir = std::env::current_dir().unwrap();
    let config_name = path.0.as_str();
    log::info!("current dir: {:?}", current_dir);
    let config: &Config = data
        .configs
        .iter()
        .find(|&x| x.name == config_name)
        .unwrap();
    log::info!("config found: {:?}", config);

    let mut cmd = Command::new("cmd");
    cmd.current_dir(&current_dir)
        .args(&[
            "/C",
            format!(
                "MultiMonitorTool.exe /TurnOn {turn_on} /SetPrimary {primary} /TurnOff {turn_off} /MoveWindow {primary} All",
                primary = config.primary,
                turn_on = config
                    .turn_on
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                    turn_off = config
                    .turn_off
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .as_str(),
        ]);
    log::info!("Executing command: {:?}", cmd);
    let output = cmd
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

#[derive(Debug, Clone)]
struct Config {
    name: String,
    turn_on: Vec<i32>,
    turn_off: Vec<i32>,
    primary: i64,
}

#[derive(Debug, Clone)]
pub struct AppState {
    configs: Vec<Config>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                configs: vec![
                    Config {
                        name: String::from("tv"),
                        turn_on: vec![3],
                        turn_off: vec![1, 2],
                        primary: 3,
                    },
                    Config {
                        name: String::from("office"),
                        turn_on: vec![1, 2],
                        turn_off: vec![],
                        primary: 1,
                    },
                ],
            })
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(setmode)
    })
    .workers(2)
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
