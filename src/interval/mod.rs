#[derive(Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

// const EMPTY: Interval = Interval {
//     min: f32::INFINITY,
//     max: f32::NEG_INFINITY,
// };

// const UNIVERSE: Interval = Interval {
//     min: f32::NEG_INFINITY,
//     max: f32::INFINITY,
// };

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }
    // pub fn contains(&self, x: f32) -> bool {
    //     self.min <= x && x <= self.max
    // }
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }
}
