use chrono::{DateTime, Datelike, Timelike, Utc};
use std::f64::consts::PI;

pub fn julian_date(dt: DateTime<Utc>) -> f64 {
    let year = dt.year() as i32;
    let month = dt.month() as i32;
    let day = dt.day() as i32;
    let hour = dt.hour() as f64 + (dt.minute() as f64) / 60.0 + (dt.second() as f64) / 3600.0;

    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;

    let jd = (day as f64)
        + (((153 * m + 2) / 5) as f64)
        + (365.0 * (y as f64))
        + ((y / 4) as f64)
        - ((y / 100) as f64)
        + ((y / 400) as f64)
        - 32045.0;

    jd + (hour - 12.0) / 24.0
}

pub fn solar_elevation(dt: DateTime<Utc>, lat: f64, lon: f64) -> f64 {
    let rad = PI / 180.0;
    let jd = julian_date(dt);
    let n = jd - 2451545.0;

    let l = (280.460 + 0.9856474 * n).rem_euclid(360.0);
    let g = (357.528 + 0.9856003 * n).rem_euclid(360.0);
    let lambda = l + 1.915 * (g * rad).sin() + 0.020 * (2.0 * g * rad).sin();
    let epsilon = 23.439 - 0.0000004 * n;

    let alpha = (epsilon * rad).cos() * (lambda * rad).sin().atan2((lambda * rad).cos()) / rad;
    let delta = ((epsilon * rad).sin() * (lambda * rad).sin()).asin() / rad;

    let gmst = (280.46061837 + 360.98564736629 * n).rem_euclid(360.0);
    let lmst = gmst + lon;
    let h = (lmst - alpha).rem_euclid(360.0);

    let lat_rad = lat * rad;
    let delta_rad = delta * rad;
    let h_rad = h * rad;

    let sin_elevation =
        lat_rad.sin() * delta_rad.sin() + lat_rad.cos() * delta_rad.cos() * h_rad.cos();
    sin_elevation.clamp(-1.0, 1.0).asin() / rad
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolarSlot {
    Dawn,
    Day,
    Dusk,
    Night,
}

impl SolarSlot {
    pub fn as_str(&self) -> &'static str {
        match self {
            SolarSlot::Dawn => "dawn",
            SolarSlot::Day => "day",
            SolarSlot::Dusk => "dusk",
            SolarSlot::Night => "night",
        }
    }
}

pub fn determine_slot(dt: DateTime<Utc>, lat: f64, lon: f64) -> (SolarSlot, f64) {
    let elev = solar_elevation(dt, lat, lon);
    let future_dt = dt + chrono::Duration::minutes(5);
    let future_elev = solar_elevation(future_dt, lat, lon);
    let is_rising = future_elev > elev;

    let slot = if elev > -0.833 {
        SolarSlot::Day
    } else if elev > -12.0 {
        if is_rising {
            SolarSlot::Dawn
        } else {
            SolarSlot::Dusk
        }
    } else {
        SolarSlot::Night
    };

    (slot, elev)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_solar_elevation_prague_noon() {
        // Summer solstice noon in Prague (high elevation)
        let dt = Utc.with_ymd_and_hms(2026, 6, 21, 11, 0, 0).unwrap();
        let elev = solar_elevation(dt, 50.0755, 14.4378);
        assert!(elev > 50.0);
        let (slot, _) = determine_slot(dt, 50.0755, 14.4378);
        assert_eq!(slot, SolarSlot::Day);
    }

    #[test]
    fn test_solar_elevation_prague_midnight() {
        // Winter midnight in Prague (deep night)
        let dt = Utc.with_ymd_and_hms(2026, 12, 21, 23, 0, 0).unwrap();
        let elev = solar_elevation(dt, 50.0755, 14.4378);
        assert!(elev < -50.0);
        let (slot, _) = determine_slot(dt, 50.0755, 14.4378);
        assert_eq!(slot, SolarSlot::Night);
    }
}
