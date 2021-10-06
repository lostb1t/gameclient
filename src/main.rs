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

pub fn run_command(mut cmd: Command) -> std::process::Output {
    log::info!("Executing command: {:?}", cmd);
    let output = cmd.output().expect("failed to execute process");
    match output.status.success() {
        true => log::info!("stdout: {}", String::from_utf8_lossy(&output.stdout)),
        false => log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr)),
    };
    output
}

pub fn spawn_command(mut cmd: Command) -> std::process::Child {
    log::info!("Executing command: {:?}", cmd);
    cmd.spawn().expect("failed to execute process")
}


pub fn kill_steam() {
    let mut cmd = Command::new("cmd");
    cmd.args(&[
        "/C",
        "taskkill.exe /F /IM steam.exe",
    ]);

    run_command(cmd);    
}

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
                "MultiMonitorTool.exe /TurnOn {turn_on} & MultiMonitorTool.exe /SetPrimary {primary} & MultiMonitorTool.exe /MoveWindow Primary All & MultiMonitorTool.exe /TurnOff {turn_off}",
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
    run_command(cmd);

    if config.start_big_picture {
        //kill_steam();
        let mut cmd = Command::new(r#"C:\Program Files (x86)\Steam\steam.exe"#);
        cmd.args(&[
            "-start", "steam://open/bigpicture"
        ]);
        spawn_command(cmd);
        //log::info!("spawned");
    }

    if config.stop_big_picture {
        kill_steam();
    }

    // log::info!("Executing command: {:?}", cmd);
    // let output = cmd
    //     .output()
    //     .expect("failed to execute process");

    // match output.status.success() {
    //     true => log::info!("stdout: {}", String::from_utf8_lossy(&output.stdout)),
    //     false => log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr)),
    // }

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
    start_big_picture: bool,
    stop_big_picture: bool,
}

#[derive(Debug, Clone)]
pub struct AppState {
    configs: Vec<Config>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=debug,actix_server=info");
    env_logger::init();

    // 1: LG monitpr
    // 2: LG TV
    // 3: Phhilips monitor
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                configs: vec![
                    Config {
                        name: String::from("tv"),
                        turn_on: vec![2],
                        turn_off: vec![1, 3],
                        primary: 2,
                        start_big_picture: true,
                        stop_big_picture: false,
                    },
                    Config {
                        name: String::from("office"),
                        turn_on: vec![1, 3],
                        turn_off: vec![0], // fake turnoff
                        primary: 1,
                        start_big_picture: false,
                        stop_big_picture: true,
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
