use gpx::Gpx;
use serde::Serialize;

const EARTH_RADIUS_IN_METERS: f64 = 6378137.0;
const ELE_THRESHOLD: f64 = 2.0;

// Struct containing all the stats related to a GpxFile
#[derive(Serialize)]
pub struct GpxStats {
    pub track_name: String,
    pub total_distance: f64,
    pub total_ascent: f64,
    pub total_descent: f64,
    pub ele_threshold_used: f64,
}

impl Default for GpxStats {
    fn default() -> Self {
        GpxStats { track_name: String::new(), total_distance: 0.0, total_ascent: 0.0, total_descent: 0.0, ele_threshold_used: ELE_THRESHOLD }
    }
}

/// Given a GPX return the complete GpxStats object with all the property
/// valorized. Uses haversine for distance calculation and to avoid noise in the 
/// total elevation we set a globale **ELE_THRESHOLD**. The higher the value the more
/// noise will be cancelled
pub fn calculate_stats(gpx: &Gpx) -> GpxStats {
    let mut stats = GpxStats::default();

    if let Some(track) = gpx.tracks.first() {
        if let Some(track_name) = &track.name {
            stats.track_name = track_name.clone();
        }
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
                                stats.total_ascent += difference;
                            } else {
                                stats.total_descent += difference.abs()
                            }
                        }
                    }
                    prev_ele = ele;
                }
                let lon2 = point.x();
                let lat2 = point.y();
                if lon1 > 0.0 && lat1 > 0.0 {
                    stats.total_distance += haversine(lat1, lon1, lat2, lon2);
                }
                lon1 = lon2;
                lat1 = lat2;
            }
        }
    }
    stats
}

/// Calculate distance between two points on a sphere using the haversine function
/// to convert, since the basic pythagorean formula calculate angles differences only.
/// **EARTH_RADIUS_IN_METERS** value has been taken from google, it should suffice for now
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

#[cfg(test)]
mod tests {
    /// Test calculate_stats using known gpx. The various consts have the correct value rounded to the nearest integer, we can use
    /// the TEST_PRECISION_BUFFER to decide in which range we want to validate (the lower the number the stricter the test)
    use super::*;
    use std::io::BufReader;
    use std::fs::File;
    use gpx::{Gpx, read};
    #[test]
    fn test_calculate_stats_simple() {
        const RIDRACOLI_DISTANCE: f64 = 32148.0;
        const RIDRACOLI_ASCENT: f64 = 2056.0;
        const RIDRACOLI_DESCENT: f64 = 2112.0;
        const TEST_PRECISION_BUFFER: f64 = 1.0;
        const RIDRACOLI_TRACK_NAME: &str = "Anello Diga di Ridracoli";

        let file = File::open("samples/ridracoli.gpx").unwrap();
        let reader = BufReader::new(file);

        let gpx: Gpx = read(reader).unwrap();
        
        let stats = calculate_stats(&gpx);

        assert_eq!(stats.ele_threshold_used, ELE_THRESHOLD);
        
        assert!(stats.total_distance > (RIDRACOLI_DISTANCE - TEST_PRECISION_BUFFER));
        assert!(stats.total_distance < (RIDRACOLI_DISTANCE + TEST_PRECISION_BUFFER));

        assert!(stats.total_ascent > (RIDRACOLI_ASCENT - TEST_PRECISION_BUFFER));
        assert!(stats.total_ascent < (RIDRACOLI_ASCENT + TEST_PRECISION_BUFFER));

        assert!(stats.total_descent > (RIDRACOLI_DESCENT - TEST_PRECISION_BUFFER));
        assert!(stats.total_descent < (RIDRACOLI_DESCENT + TEST_PRECISION_BUFFER));

        assert_eq!(stats.track_name, RIDRACOLI_TRACK_NAME);
    }
}
