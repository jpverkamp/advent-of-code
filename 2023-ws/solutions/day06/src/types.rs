#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub record: u64,
}

impl Race {
    pub fn record_breakers_bf(&self) -> u64 {
        (0..=self.time)
            .filter(|x| x * (self.time - x) > self.record)
            .count() as u64
    }

    pub fn record_breakers(&self) -> u64 {
        // Race is D units long
        // Each option is hold the button for x seconds, maximum of T
        // Distance traveled is x for T-x seconds
        // We need to travel at least D
        // x(T-x) > D
        // xT - x^2 > D
        // x^2 - xT + D < 0
        // x in (T +/- sqrt(T^2 - 4D)) / 2

        let t = self.time as f64;
        let d = self.record as f64;

        let x1 = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
        let x2 = (t + (t * t - 4.0 * d).sqrt()) / 2.0;

        let lo = x1.min(x2).ceil() as u64;
        let hi = x1.max(x2).floor() as u64;

        // If lo is an integer, we don't want it (< vs <=)
        // But it's a float, so check by epsilon difference
        // This isn't perfect, but it works
        let diff = ((lo as f64) - x1.min(x2)).abs();

        if diff < 1e-6 {
            hi - lo - 1
        } else {
            hi - lo + 1
        }
    }
}
