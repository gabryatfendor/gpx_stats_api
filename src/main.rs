use actix_web::{App, HttpResponse, HttpServer, Responder, post};
use gpx::read;
use std::io::Cursor;

const EARTH_RADIUS_IN_METERS: f64 = 6378137.0;
const ELE_THRESHOLD: f64 = 2.0;

#[post("/upload")]
async fn upload(req_body: String) -> impl Responder {
    let cursor = Cursor::new(req_body);
    match read(cursor) {
        Ok(gpx) => {
            let mut total_distance = 0.0;
            let mut total_ascent = 0.0;
            let mut total_descent = 0.0;

            if let Some(track) = gpx.tracks.first() {
                let mut prev_ele = 0.0;
                let mut lon1 = 0.0;
                let mut lat1 = 0.0;
                for segment in &track.segments {
                    for waypoint in &segment.points {
                        let point = waypoint.point();
                        if let Some(ele) = waypoint.elevation {
                            if prev_ele > 0.0 {
                                let difference = ele - prev_ele;
                                if difference.abs() > ELE_THRESHOLD {
                                    if difference >= 0.0 {
                                        total_ascent = total_ascent + (difference)
                                    } else {
                                        total_descent = total_descent + (difference.abs())
                                    }
                                }
                            }
                            prev_ele = ele;
                        }
                        let lon2 = point.x();
                        let lat2 = point.y();
                        if lon1 > 0.0 && lat1 > 0.0 {
                            total_distance = total_distance + haversine(lat1, lon1, lat2, lon2);
                        }
                        lon1 = lon2;
                        lat1 = lat2;
                    }
                }
            }
            HttpResponse::Ok().body(format!("Total distance: {:.2} km\n\nTotal ascent: {:.2} m\n\nTotal descent: {:.2} m\n\nElevation calculated with threshold of {:.2} meters", total_distance/1000.0, total_ascent, total_descent, ELE_THRESHOLD))
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Error in GPX reading: {}", e)),
    }
}

fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS_IN_METERS * c
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(upload))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
