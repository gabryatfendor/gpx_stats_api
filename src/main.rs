mod stats;

use actix_web::{App, HttpResponse, HttpServer, Responder, post};
use gpx::read;
use stats::calculate_stats;
use std::io::Cursor;

#[post("/upload")]
async fn upload(req_body: String) -> impl Responder {
    let cursor = Cursor::new(req_body);

    match read(cursor) {
        Ok(gpx) => {
            let stats = calculate_stats(&gpx);
            HttpResponse::Ok().body(format!(
                "Track name: {}\n, Total distance: {:.2} km\nTotal ascent: {:.2} m\nTotal descent: {:.2} m\n\nElevation calculated with threshold of {:.2} meters", 
                stats.track_name,
                stats.total_distance/1000.0,
                stats.total_ascent,
                stats.total_descent,
                stats.ele_threshold_used))
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Error in GPX reading: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(upload))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
