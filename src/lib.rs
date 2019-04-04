//! # secfmt
//!
//! `secfmt` converts seconds into a human readable format (struct) containing years, days, hours, minutes and seconds.

#[derive(Debug)]
pub struct Secfmt {
    pub years: u8,
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

/// Converts seconds into a human readable format (struct) containing years, days, hours, minutes and seconds.
///
/// # Examples
///
/// ```
/// let seconds = 31537529;
/// let seconds_human_readable = secfmt::from(seconds);
///
/// assert_eq!(1, secfmt::from(31537529).years);
/// assert_eq!(0, secfmt::from(31537529).days);
/// assert_eq!(0, secfmt::from(31537529).hours);
/// assert_eq!(25, secfmt::from(31537529).minutes);
/// assert_eq!(29, secfmt::from(31537529).seconds);
/// ```
pub fn from(s: u64) -> Secfmt {
    let (mut days, mut hours, mut minutes, mut seconds) = (0, 0, 0, 0);

    let years = s / 31536000;
    let mut remainder = s % 31536000;
    if remainder > 0 {
        days = remainder / 86400;
        remainder = remainder % 86400;
        if remainder > 0 {
            hours = remainder / 3600;
            remainder = remainder % 3600;
            if remainder > 0 {
                minutes = remainder / 60;
                seconds = remainder % 60;
            }
        }
    }

    Secfmt {
        years: years as u8,
        days: days as u8,
        hours: hours as u8,
        minutes: minutes as u8,
        seconds: seconds as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_years() {
        assert_eq!(1, from(31537529).years);
    }

    #[test]
    fn test_days() {
        assert_eq!(0, from(31537529).days);
    }

    #[test]
    fn test_hours() {
        assert_eq!(0, from(31537529).hours);
    }

    #[test]
    fn test_minutes() {
        assert_eq!(25, from(31537529).minutes);
    }

    #[test]
    fn test_seconds() {
        assert_eq!(29, from(31537529).seconds);
    }
}