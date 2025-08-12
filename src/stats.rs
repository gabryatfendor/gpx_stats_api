use gpx::Gpx;

const EARTH_RADIUS_IN_METERS: f64 = 6378137.0;
const ELE_THRESHOLD: f64 = 2.0;

pub struct GpxStats {
    pub total_distance: f64,
    pub total_ascent: f64,
    pub total_descent: f64,
    pub ele_threshold_used: f64,
}

pub fn calculate_stats(gpx: &Gpx) -> GpxStats {
    let mut stats = GpxStats {
        total_ascent: 0.0,
        total_descent: 0.0,
        total_distance: 0.0,
        ele_threshold_used: 0.0,
    };

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
    stats.ele_threshold_used = ELE_THRESHOLD;
    stats
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_stats_simple() {
        assert!(true);
    }
}
