use std::io::Cursor;

use gpx::{Gpx, Track, TrackSegment, read, write};

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Are you looking for something?")
}

#[post("/upload")]
async fn upload(req_body: String) -> impl Responder {
    let cursor = Cursor::new(req_body);
    match read(cursor) {
        Ok(gpx) => {
            if let Some(track) = gpx.tracks.first() {
                println!("Track name: {:?}", track.name);
                for segment in &track.segments {
                    for waypoint in &segment.points {
                        let point = waypoint.point();
                        println!("Latitude: {}, Longitude: {}", point.x(), point.y());
                        if let Some(ele) = waypoint.elevation {
                            println!("Elevation: {ele}");
                        } 
                    }
                }
            }
            HttpResponse::Ok().body("GPX file succesfully uploaded")
        }
        Err(e) => {
            HttpResponse::BadRequest().body(format!("Error in GPX reading: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(default)
            .service(upload)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}