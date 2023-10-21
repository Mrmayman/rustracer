const INFINITY: f64 = f64::INFINITY;
const NEG_INFINITY: f64 = f64::NEG_INFINITY;

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(_min: f64, _max: f64) -> Self {
        Interval {
            min: _min,
            max: _max,
        }
    }

    pub fn new_interval(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
        }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding: f64 = delta/2.0;
        return Interval::new(self.min - padding, self.max + padding);
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }
}

static EMPTY: Interval = Interval {
    min: INFINITY,
    max: NEG_INFINITY,
};

static UNIVERSE: Interval = Interval {
    min: NEG_INFINITY,
    max: INFINITY,
};

impl std::ops::Add<f64> for Interval {
    type Output = Interval;
    fn add(self, other: f64) -> Interval {
        Interval {
            min: self.min + other,
            max: self.max + other,
        }
    }
}