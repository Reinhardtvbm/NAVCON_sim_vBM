use std::{time::Instant, f32::consts::PI};

const R: f32 = 60.0;

/*===================ENUMS=======================*/
enum States {
    Stop,
    Forward,
    Reverse,
    TurnRight,
    TurnLeft,
    MazeDone
}

#[derive(Clone, Copy, PartialEq)]
pub enum Colours {
    Red,
    White,
    Green,
    Blue,
    Black,
}

struct Mdps {
    speed_l: i8,
    speed_r: i8,
    distance_l: f32,
    distance_r: f32,
    time: Instant
}

impl Mdps {
    fn new() -> Self {
        Self { speed_l: 0, speed_r: 0,  distance_l: 0.0, distance_r: 0.0, time: Instant::now() }
    }

    fn update(&mut self) {
        self.distance_l = (self.speed_l as f32)*(self.time.elapsed().as_millis() as f32/1000.0);
        self.distance_r = (self.speed_r as f32)*(self.time.elapsed().as_millis() as f32/1000.0);
        //self.time = Instant::now();
    }

    fn send_stop(&mut self) {
        println!("distance: {}, {} {} {} ", self.distance_l, self.speed_l, self.speed_r, self.time.elapsed().as_millis() as f32/1000.0);
        self.distance_r = 0.0;
        self.distance_l = 0.0;
        self.speed_r = 0;
        self.speed_l = 0;

        println!("Waiting for MARV to stop...");
    }

    fn send_motor_speeds(&mut self, v_l: i8, v_r: i8) {
        self.speed_l = v_l*10;
        self.speed_r = v_r*10;
    }
}

impl std::fmt::Display for Colours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colours::Red => write!(f, "Red"),
            Colours::White => write!(f, "White"),
            Colours::Green => write!(f, "Green"),
            Colours::Blue => write!(f, "Blue"),
            Colours::Black => write!(f, "Black"),
        }
    }
}

struct NavconSim {
    c: [Colours; 5],    // colour sensors
    a: i8,             // incidence
    b: bool
}

impl NavconSim {
    fn print(&self) {
        println!("NAVCON Simulation/test bench:");
        println!("Angle Af Incidence: {} degrees", self.a);
        println!("Colour Sensor: [{}, {}, {}, {}, {}] ", self.c[0], self.c[1], self.c[2], self.c[3], self.c[4]);
    }
}
/*===============================================*/

fn main() {
    let maze_done = false;
    
    let mut navcon_sim = NavconSim{ c: [Colours::Blue; 5], a: 0,b: false };

    let mut mdps = Mdps::new();

    //while !maze_done {
        //todo!(); // get input data!
        
        navcon_sim.print();

        mdps.send_stop();

        mdps.send_motor_speeds(10, 10);
        println!("MARV going forward!");

        while navcon_sim.c == [Colours::White; 5] {}

        if !navcon_sim.c.contains(&Colours::Green) {
            reverse(&mut mdps, &mut navcon_sim);
        //}
    }
}

fn reverse(mdps: &mut Mdps, nav_sim: &mut NavconSim) {
    println!("MARV reversing...");
    mdps.send_motor_speeds(-10, -10);

    if nav_sim.a.abs() < 45 {
        if nav_sim.c.contains(&Colours::Green) {
            if nav_sim.a > 0 {
                rotate_left(mdps, nav_sim.a.abs() as u8);
            }
            else {
                rotate_right(mdps, nav_sim.a.abs() as u8);
            }
        }
        else {
            if nav_sim.b == false {
                nav_sim.b = true;
                rotate_right(mdps, 90-nav_sim.a.abs() as u8);
            }
            else {
                nav_sim.b = false;
                rotate_right(mdps, 180-nav_sim.a.abs() as u8);
            }
        }
    }
    else {
        if nav_sim.c.contains(&Colours::Green) {
            if nav_sim.a > 0 {
                rotate_right(mdps, 5);
            }
            else {
                rotate_left(mdps, 5);
            }
        }
        else {
            if nav_sim.a > 0 {
                rotate_left(mdps, 5);
            }
            else {
                rotate_right(mdps, 5);
            }
        }
    }
}

fn rotate_right(mdps: &mut Mdps, angle: u8) {
    println!("MARV rotating right!, {}", angle);

    let dist = (angle as f32)*(PI/180.0)*R;

    mdps.send_stop();
    mdps.send_motor_speeds(10, -10);

    while mdps.distance_l < dist {
        mdps.update();
    }

    mdps.send_stop();
}

fn rotate_left(mdps: &mut Mdps, angle: u8) {
    println!("MARV rotating left!, {}", angle);

    let dist = (angle as f32)*(PI/180.0)*R;

    mdps.send_stop();
    mdps.send_motor_speeds(-10, 10);

    while mdps.distance_l < dist {
        mdps.update();
    }

    mdps.send_stop();
}

