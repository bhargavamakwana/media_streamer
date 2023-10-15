use std::fs::File;
use std::io::Read;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer};
use tokio::io::AsyncReadExt;

#[get("/video/stream")]
async fn video_stream(req: HttpRequest) -> HttpResponse {
    let video_path = "/home/bhargav/Downloads/American_Made/AmericanMade.mp4"; // Replace with your video file path

    match File::open(video_path){
        Ok(mut file) => {
            let mut video_data = Vec::new();
            if let Err(_) = file.read_to_end(&mut video_data){
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok()
                .header("Content-Type", "video/mp4")
                .body(video_data)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(video_stream))
        .bind("0.0.0.0:3030")?
        .run()
        .await
}
