use std::{time::Instant, f32::consts::PI};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::fs;
mod test_data;
use test_data::TestData;
const R: f32 = 60.0;

/*===================ENUMS=======================*/
#[derive(Clone, Copy, PartialEq, EnumIter)]
#[derive(Debug)]
pub enum Colours {
    Red,
    White,
    Green,
    Blue,
    Black,
}

struct Mdps {
    speed_l: i32,
    speed_r: i32,
    distance_l: f32,
    distance_r: f32,
    time: Instant
}

impl Mdps {
    fn new() -> Self {
        Self { speed_l: 0, speed_r: 0,  distance_l: 0.0, distance_r: 0.0, time: Instant::now() }
    }

    fn update(&mut self) {
        self.distance_l = (self.speed_l as f32)*(self.time.elapsed().as_nanos() as f32/1000_000_000.0);
        self.distance_r = (self.speed_r as f32)*(self.time.elapsed().as_nanos() as f32/1000_000_000.0);
    }

    fn send_stop(&mut self) -> String {
        let mut s = format!("Distance since last stop: {} mm\nleft wheel velocity: {} mm/s\nRight wheel velocity: {} mm/s\nTime since last stop: {} s ", self.distance_l, self.speed_l, self.speed_r, self.time.elapsed().as_nanos() as f32/1000_000_000.0);
        self.distance_r = 0.0;
        self.distance_l = 0.0;
        self.speed_r = 0;
        self.speed_l = 0;
        self.time = Instant::now();
        s = format!("{}\n\nWaiting for MARV to stop...\n", s);
        s
    }

    fn send_motor_speeds(&mut self, v_l: i32, v_r: i32) {
        self.speed_l = v_l*1000;
        self.speed_r = v_r*1000;
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
    fn print(&self) -> String {
        let mut s = String::new();

        s = format!("{}\nNAVCON Simulation/test bench:", s);
        s = format!("{}\nAngle Of Incidence: {} degrees", s, self.a); 
        s = format!("{}\nColour Sensor: [{}, {}, {}, {}, {}] ", s, self.c[0], self.c[1], self.c[2], self.c[3], self.c[4]);
        
        s
    }
}
/*===============================================*/

fn main() {
    let mut test_data: Vec<TestData> = Vec::new();
    populate_test_data(&mut test_data);

    let mut results = String::new();
    
    let mut navcon_sim = NavconSim{ c: [Colours::Blue; 5], a: 0,b: false };

    let mut mdps = Mdps::new();
    
    let mut i = 1;
    
    for data in test_data {
        navcon_sim.a = data.aoc;
        navcon_sim.c = data.col_sens;

        results = format!("{}\n{}", results, navcon_sim.print());

        results = format!("{}\n{}", results, mdps.send_stop());

        if navcon_sim.c != [Colours::White; 5] {
            if navcon_sim.c.contains(&Colours::Green) {
                if navcon_sim.a.abs() < 5 {
                    mdps.send_motor_speeds(10, 10);
                    results = format!("{}\nMARV going forward!", results);
                }
                else {
                    results = format!("{}\n{}", results, reverse(&mut mdps, &mut navcon_sim));
                }
            }
            else {
                if navcon_sim.c.contains(&Colours::Red) {
                    if navcon_sim.a.abs() < 5 {
                        mdps.send_motor_speeds(10, 10);
                        results = format!("{}\nMARV going forward!", results);
                        results = format!("{}\nMaze finished!!", results);
                    }
                    else {
                        results = format!("{}\n{}", results, reverse(&mut mdps, &mut navcon_sim));
                    }
                }
                else {
                    results = format!("{}\n{}", results, reverse(&mut mdps, &mut navcon_sim));
                }
            }
        }
        else {
            results = format!("{}\nMARV going forward!", results);
        }
        results = format!("{}\n================================================================\n\n", results);
        println!("{}", i);
        i += 2;
    }

    fs::write("results.txt", results).expect("file no work");
}

fn reverse(mdps: &mut Mdps, nav_sim: &mut NavconSim) -> String {
    let mut s = String::new();
    mdps.send_stop();
    s = format!("{}\nMARV reversing...", s);
    mdps.send_motor_speeds(-10, -10);

    if nav_sim.a.abs() < 45 {
        if nav_sim.c.contains(&Colours::Green) {
            if nav_sim.a > 0 {
                let s1 = rotate_left(mdps, nav_sim.a.abs() as u8);
                s = format!("{}\n{}", s, s1);
            }
            else {
                let s1 =  rotate_right(mdps, nav_sim.a.abs() as u8);
                s = format!("{}\n{}", s, s1);
            }
        }
        else {
            if nav_sim.b == false {
                nav_sim.b = true;
                let s1 =rotate_right(mdps, 90-nav_sim.a.abs() as u8);
                s = format!("{}\n{}", s, s1);
            }
            else {
                nav_sim.b = false;

                let s1 =rotate_right(mdps, 180-nav_sim.a.abs() as u8);
                s = format!("{}\nsecond time seeing blue/black\n{}", s, s1);
            }
        }
    }
    else {
        if nav_sim.c.contains(&Colours::Green) {
            if nav_sim.a > 0 {
                let s1 =rotate_right(mdps, 5);
                s = format!("{}\n{}", s, s1);
            }
            else {
                let s1 =rotate_left(mdps, 5);
                s = format!("{}\n{}", s, s1);
            }
        }
        else {
            if nav_sim.a > 0 {
                let s1 = rotate_left(mdps, 5);
                s = format!("{}\n{}", s, s1);
            }
            else {
                let s1 =rotate_right(mdps, 5);
                s = format!("{}\n{}", s, s1);
            }
        }
    }
    s
}

fn rotate_right(mdps: &mut Mdps, angle: u8) -> String {
    let mut s = String::new();

    

    let dist = (angle as f32)*(PI/180.0)*R;

    s = format!("{}\n{}", s, mdps.send_stop());
    mdps.send_motor_speeds(10, -10);

    s = format!("{}\nMARV rotating right!, {}", s, angle);

    while mdps.distance_l.abs() < dist {
        mdps.update();
    }

    s = format!("{}\n{}", s, mdps.send_stop());
    s
}

fn rotate_left(mdps: &mut Mdps, angle: u8) -> String {
    let mut s = String::new();
    
    let dist = (angle as f32)*(PI/180.0)*R;
    s = format!("{}\n{}", s, mdps.send_stop());
    mdps.send_motor_speeds(-10, 10);
    s = format!("{}\nMARV rotating left!, {}", s, angle);
    while mdps.distance_l.abs() < dist {
        mdps.update();
    }
    s = format!("{}\n{}", s, mdps.send_stop());
    s
}

fn populate_test_data(data: &mut Vec<TestData>) {
    let mut aois = Vec::new();

    for i in -90..90 {
        aois.push(i as i8);
    }

    let mut colours = Vec::new();

    for col0 in Colours::iter() {
        for col1 in Colours::iter() {
            for col2 in Colours::iter() {
                for col3 in Colours::iter() {
                    for col4 in Colours::iter() {
                        if (col0 == col1 || col1 == Colours::White || col0 == Colours::White) && (col2 == Colours::White && col3 == Colours::White && col4 == Colours::White) { // col2 == Colours::White && col3 == Colours::White && col4 == Colours::White
                            let new_array = [col0, col1, col2, col3, col4];
                            colours.push(new_array);
                        }
                    }
                }
            }
        }
    }

    for angle in aois {
        for col in &colours {
            data.push(TestData::new(*col, angle));
        }
    }

    
    let mut test_data = String::new();
    
    for d in data {
        test_data = format!("{}Angle of Incidence {}\nColour Sensor: [{}, {}, {}, {}, {}]\n",test_data, d.aoc.to_string(), d.col_sens[0].to_string(), d.col_sens[1].to_string(), d.col_sens[2].to_string(), d.col_sens[3].to_string(), d.col_sens[4].to_string());
    }

    fs::write("test_data.txt", test_data).expect("file no work");
}


