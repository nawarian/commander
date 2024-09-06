use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::http::header;
use serde::Deserialize;
use std::process::{Command, Stdio};
use tokio::task;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                actix_cors::Cors::permissive()
                    .allowed_origin_fn(|_origin, _req_head| true)
            )
            .route("/whoami", web::get().to(whoami))
            .route("/youtube", web::post().to(launch_youtube))
    })
    .bind("0.0.0.0:7171")?
    .run()
    .await
}

async fn whoami() -> impl Responder {
    let output = Command::new("whoami")
        .output()
        .expect("failed to execute process");

    let whoami = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    HttpResponse::Ok().json(whoami)
}

#[derive(Deserialize)]
struct LaunchYoutubeQueryParams {
    video: Option<String>,
    format: Option<String>,
}

async fn launch_youtube(req: HttpRequest) -> impl Responder {
    let params: web::Query<LaunchYoutubeQueryParams> = web::Query::from_query(req.query_string()).unwrap();

    let video = params.video.clone().unwrap().to_string();
    let format = params.format.clone().unwrap().to_string();

    println!("Launching youtube video {video} with format {format}");

    let handle = task::spawn(async move {
        let _ = Command::new("/usr/bin/mpv")
            .arg(format!("https://www.youtube.com/watch?v={video}", video=video))
            // .arg(format!("--ytdl-format={format}", format=format))
            .arg("--no-ytdl")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    });

    match handle.await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
