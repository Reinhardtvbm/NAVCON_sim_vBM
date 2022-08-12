const HALF_BLOCK: f32 = 10 as f32;

use crate::colours::Colours;
pub struct Navcon {
    /* INPUTS */
    pub colour_sensor: [Colours; 5],
    pub incidence: u16,
    speed: f32,
    distance: f32,

    /* OUTPUTS */
    velocity_l: i32,
    velocity_r: i32,

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
            velocity_l: 0 as i32, 
            velocity_r: 0 as i32,
            transmission: false,
        }
    }

    pub fn colours_in(&mut self, colours: [Colours; 5]) {
        self.colour_sensor = colours;
    }

    pub fn incidence_in(&mut self, angle: u16) {
        self.incidence = angle;
    }

    pub fn speed_in(&mut self, s: f32) {
        self.speed = s;
    }

    pub fn distance_in(&mut self, dis: f32) {
        self.distance = dis;
    }

    pub fn reverse(&mut self, distance: f32) {
        while self.distance < distance {
            self.velocity_l = -5;
            self.velocity_r = -5;
        }
    }

    pub fn forward(&mut self) {
        self.velocity_l = 5;
        self.velocity_r = 5;
    }

    pub fn stop(&mut self) {
        while self.speed != 0.0 {
            self.velocity_l = 0;
            self.velocity_r = 0;
        }
    }

    pub fn turn_90_cw(&mut self) {
        todo!();
    }

    pub fn turn_90_ccw(&mut self) {
        todo!();
    }

    pub fn turn_180(&mut self) {
        todo!();
    }

    pub fn turn_360(&mut self) {
        todo!();
    }

    pub fn turn_5_cw(&mut self) {
        todo!()
    }

    pub fn correction(&mut self) {
        todo!();
    }

    pub fn b_n_encountered(&mut self) {
        let stop = false;
        
        if self.incidence < 5 {
            self.reverse(HALF_BLOCK);
            self.stop();
            self.turn_90_cw();
            self.forward();
        }
        else {
            while !stop {
                self.reverse(HALF_BLOCK/2.0);
                self.stop();
                self.turn_5_cw();
                
            }
            self.forward();
        }
    }

    pub fn green_encountered(&mut self) {
        // while incidence > 45, do 5 degree turns until under
        while self.incidence > 45 {
            while !(self.incidence < 45)  {
                self.stop();
                self.reverse(HALF_BLOCK/4.0);
                self.stop();
                self.turn_5_cw();

                while !self.colour_sensor.contains(&Colours::Green){
                    self.forward();
                }
            }
        }

        // while incidence > 5, do steering correction
        while self.incidence > 5 {
            self.stop();
            self.reverse(HALF_BLOCK/2.0);
            self.stop();
            self.correction();

            while !(self.colour_sensor.contains(&Colours::Green)) {
                self.forward();
            }
        }

        // if incidence good, then go forward :)
        if self.incidence < 5 {
            self.forward();
        }
    }

    pub fn contains_non_white(&self) -> bool {
        self.colour_sensor.contains(&Colours::Black) || self.colour_sensor.contains(&Colours::Blue) || self.colour_sensor.contains(&Colours::Green) || self.colour_sensor.contains(&Colours::Red)
    }
}

pub fn transmit_colours(nav: &mut Navcon, colours: [Colours; 5]) {
    nav.colours_in(colours);
    nav.transmission = true;
}

pub fn transmit_incidence(nav: &mut Navcon, incidence: u16) {
    nav.incidence_in(incidence);
    nav.transmission = true;
}

pub fn transmit_speed(nav: &mut Navcon, speed: f32) {
    nav.speed_in(speed);
    nav.transmission = true;
}

pub fn transmit_distance(nav: &mut Navcon, distance: f32) {
    nav.distance_in(distance);
    nav.transmission = true;
}