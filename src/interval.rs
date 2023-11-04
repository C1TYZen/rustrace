pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            min: x,
            max: y,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min };
        if x > self.max { return self.max };
        return x;
    }
}

// const Empty: Interval = Interval{+infinity, -infinity};
// const static interval universe(-infinity, +infinity);
