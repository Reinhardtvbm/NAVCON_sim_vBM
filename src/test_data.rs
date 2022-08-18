use crate::Colours;

pub struct TestData {
    // Colour Sensor
    pub col_sens: [Colours; 5],

    // Angle of Incidence
    pub aoc: i8,
}

impl TestData {
    pub fn new(c: [Colours; 5], a: i8) -> Self {
        Self { 
            col_sens: c, 
            aoc: a, 
        }
    }
}