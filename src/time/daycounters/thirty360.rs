use super::traits::DoesDayCount;
use crate::time::date::Date;
pub struct Thirty360;

impl DoesDayCount for Thirty360 {
    fn day_count(&self, start: Date, end: Date) -> i64 {
        let mut d1 = start.day() as i32;
        let mut d2 = end.day() as i32;
        let mut m1 = start.month() as i32;
        let mut m2 = end.month() as i32;
        let mut y1 = start.year();
        let mut y2 = end.year();

        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 {
            d2 = 30;
        }

        if d1 == 30 && d2 == 30 {
            d2 = 31;
        }

        if d1 == 30 && d2 == 31 {
            d2 = 1;
            m2 += 1;
        }

        if d1 == 31 && d2 == 30 {
            d2 = 1;
            m2 += 1;
        }

        if d1 == 31 && d2 == 31 {
            d2 = 1;
            m2 += 1;
        }

        if m1 == 12 && m2 == 1 {
            m2 = 12;
            y2 -= 1;
        }

        return (360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)) as i64;
    }

    fn year_fraction(&self, start: Date, end: Date) -> f64 {
        return self.day_count(start, end) as f64 / 360.0;
    }
}
