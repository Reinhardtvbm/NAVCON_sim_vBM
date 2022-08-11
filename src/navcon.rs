const HALF_BLOCK: f32 = 10 as f32;

#[derive(Copy, Clone, PartialEq)]
enum Colours {
    White, Black, Red, Green, Blue
}

struct Navcon {
    /* INPUTS */
    colour_sensor: [Colours; 5],
    incidence: u16,
    speed: f32,
    distance: f32,

    /* OUTPUTS */
    velocity_l: i32,
    velocity_r: i32,
}

impl Navcon {
    fn new() -> Self {
        Self { 
            colour_sensor: [Colours::White; 5], 
            incidence: 0,
            speed: 0 as f32,
            distance: 0 as f32,
            velocity_l: 0 as i32, 
            velocity_r: 0 as i32
        }
    }

    fn sensor_in(&mut self, colours: [Colours; 5]) {
        self.colour_sensor = colours;
    }

    fn incidence_in(&mut self, angle: u16) {
        self.incidence = angle;
    }

    fn reverse(&mut self, distance: f32) {
        while self.distance < distance {
            self.velocity_l = -5;
            self.velocity_r = -5;
        }
    }

    fn forward(&mut self) {
        self.velocity_l = 5;
        self.velocity_r = 5;
    }

    fn stop(&mut self) {
        while self.speed != 0.0 {
            self.velocity_l = 0;
            self.velocity_r = 0;
        }
    }

    fn turn_90_cw(&mut self) {
        todo!();
    }

    fn turn_90_ccw(&mut self) {
        todo!();
    }

    fn turn_180(&mut self) {
        todo!();
    }

    fn turn_360(&mut self) {
        todo!();
    }

    fn turn_5_cw(&mut self) {
        todo!()
    }

    fn correction(&mut self) {
        todo!();
    }

    fn b_n_encountered(&mut self) {
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

    fn green_encountered(&mut self) {
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
}