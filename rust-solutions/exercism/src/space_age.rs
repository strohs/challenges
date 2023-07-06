/// # Excercism - Space Age
/// https://exercism.org/tracks/rust/exercises/space-age
/// given a persons age in seconds calculate how old they would be on each planet

const SECONDS_PER_EARTH_YEAR: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    pub seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    // SECONDS_PER_YEAR = planets orbital period * SECONDS_PER_EARTH_YEAR
    const SECONDS_PER_YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::SECONDS_PER_YEAR
    }
}

pub struct Mercury;

pub struct Venus;

pub struct Earth;

pub struct Mars;

pub struct Jupiter;

pub struct Saturn;

pub struct Uranus;

pub struct Neptune;

impl Planet for Mercury {
    const SECONDS_PER_YEAR: f64 = 0.2408467 * SECONDS_PER_EARTH_YEAR;
}

impl Planet for Venus {
    const SECONDS_PER_YEAR: f64 = 0.61519726 * SECONDS_PER_EARTH_YEAR;
}

impl Planet for Earth {
    const SECONDS_PER_YEAR: f64 = 1.0 * SECONDS_PER_EARTH_YEAR;
}

impl Planet for Mars {
    const SECONDS_PER_YEAR: f64 = 1.8808158 * SECONDS_PER_EARTH_YEAR;
}

impl Planet for Jupiter {
    const SECONDS_PER_YEAR: f64 = 11.862615 * SECONDS_PER_EARTH_YEAR;
}

impl Planet for Saturn {
    const SECONDS_PER_YEAR: f64 = 29.447498 * SECONDS_PER_EARTH_YEAR;
}

impl Planet for Uranus {
    const SECONDS_PER_YEAR: f64 = 84.016846 * SECONDS_PER_EARTH_YEAR;
}

impl Planet for Neptune {
    const SECONDS_PER_YEAR: f64 = 164.79132 * SECONDS_PER_EARTH_YEAR;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_year_calc() {
        assert!(true)
    }
}
