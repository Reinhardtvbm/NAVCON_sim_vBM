const HALF_BLOCK: f32 = 10.0;
const R: f32 = 50.0; // half axle lenght in mm

use std::{thread, time};



use crate::colours::Colours;
pub struct Navcon {
    /* INPUTS */
    pub colour_sensor: [Colours; 5],
    pub incidence: u16,
    speed: f32,
    pub distance: f32,

    /* OUTPUTS */
    velocity_l: f32,
    velocity_r: f32,

    /* CONTROL */
    pub transmission: bool,
}

impl Navcon {
    pub fn new() -> Self {
        Self { 
            colour_sensor: [Colours::White; 5], 
            incidence: 0,
            speed: 0 as f32,
            distance: 0 as f32,
            velocity_l: 0 as f32, 
            velocity_r: 0 as f32,
            transmission: false,
        }
    }

    pub fn set_straight(&mut self, x: f32) {
        self.velocity_l = x;
        self.velocity_r = x; 
        
        if x > 0.0 {
            println!("MARV going forward!");
        }
        else {
            println!("MARV reversing!");
        }
        
    }

    pub fn set_stop(&mut self) {
        self.velocity_l = 0.0;
        self.velocity_r = 0.0;
        println!("MARV stopped!");
    }

    pub fn start_left_turn(&mut self, angle: f32, x: f32) -> f32 {
        // set wheel velocities for a left turn
        self.velocity_r = x;
        self.velocity_l = -x;

        // return the arc length that must be travelled to achieve the left turn for specified angle
        R*angle
    }

    pub fn start_right_turn(&mut self, angle: f32, x: f32) -> f32 {
        // set wheel velocities for a right turn
        self.velocity_r = -x;
        self.velocity_l = x;

        // return the arc length that must be travelled to achieve the left turn for specified angle
        R*angle
    }

    // simulated wait for input
    pub fn wait_for_input(&self) {
        println!("waiting for input...");
        thread::sleep(time::Duration::from_millis(500));
        println!("got input!");
    }

    pub fn interperet_colours(&self) -> (bool, bool, bool) {
        if self.colour_sensor.contains(&Colours::Green) || self.colour_sensor.contains(&Colours::Red) {
            return (true, false, false);
        }
        else if self.colour_sensor.contains(&Colours::Green) || self.colour_sensor.contains(&Colours::Red) {
            return (false, true, false);
        }
        else {
            return (false, false, true);
        }
    }
}