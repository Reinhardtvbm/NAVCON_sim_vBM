use crate::Colours;

pub struct TestData {
    // Colour Sensor
    pub col_sens: [Colours; 5],

    // Angle of Incidence
    pub aoc: i32,

    // Distance
    pub dist: f32,

    // Speed
    pub speed: f32
}

impl TestData {
    pub fn new(c: [Colours; 5], a: i32, d: f32, s: f32) -> Self {
        Self { 
            col_sens: c, 
            aoc: a, 
            dist: d, 
            speed: s
        }
    }
}