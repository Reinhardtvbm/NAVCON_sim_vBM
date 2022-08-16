mod test_data;
use test_data::TestData;
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
    c: [Colours; 5],    // colour sensor
    d: f32,             // distance
    s: f32,             // speed
    a: i32,             // incidence

    reverse: bool,
    t_angle: i32
}

impl NavconSim {
    fn print(&self) {
        println!("NAVCON Simulation/test bench:");
        println!("Angle Af Incidence: {} degrees", self.a);
        println!("Colour Sensor: [{}, {}, {}, {}, {}] ", self.c[0], self.c[1], self.c[2], self.c[3], self.c[4]);
        println!("Distance: {} mm", self.d);
        println!("Speed: {} mm/s", self.s);
    }
}
/*===============================================*/

fn main() {
    let mut s = States::Stop;

    let mut navcon_sim = NavconSim {
        c: [Colours::White; 5],
        d: 0.0,
        s: 0.0,
        a: 0,
        reverse: false,
        t_angle: 0
    };
    let mut test_data: Vec<TestData> = Vec::new();
    let mut ignore_input = false;
    init_test_data(&mut test_data);
    let mut i = 0_usize;
    let mut cont = true;

    while cont {
        if !ignore_input {
            navcon_sim.print();
            println!("");
        }
        match s {
            States::Stop => stop(&mut navcon_sim, &mut s),
            States::Forward => forward(&mut navcon_sim, &mut s, &mut cont, &mut ignore_input),
            States::Reverse => reverse(&mut navcon_sim, &mut s),
            States::TurnRight => {
                turn_right(&mut navcon_sim, &mut s);
                ignore_input = false;
            },
            States::TurnLeft => {
                turn_left(&mut navcon_sim, &mut s);
                ignore_input = false;
            },
            States::MazeDone => println!("Maze done!"),
        }
        
        if !ignore_input {
            println!("END STATE-------------------------------------------------------\n");
            navcon_sim.a = test_data.get(i).expect("test data done...").aoc;
            navcon_sim.c = test_data.get(i).expect("test data done...").col_sens;
            navcon_sim.d = test_data.get(i).expect("test data done...").dist;
            navcon_sim.s = test_data.get(i).expect("test data done...").speed;
        
            i += 1;
        }
    }   
}

fn stop(sim: &mut NavconSim, state: &mut States) {
    println!("State: Stop");
    println!("sending stop command to MDPS...\n");
    if sim.c == [Colours::White; 5] {
        *state = States::Forward;
        return;
    }

    if sim.c.contains(&Colours::Green) && sim.a.abs() < 5 {
        *state = States::Forward;
        return;
    }

    *state = States::Reverse;
}

fn forward(sim: &mut NavconSim, state: &mut States, continue_maze: &mut bool, ignore_input: &mut bool) {
    println!("State: Forward");
    println!("MARV zooming straight ahead!\n");
    if sim.c == [Colours::White; 5] {
        *state = States::Forward;
        return;
    }

    if sim.c.contains(&Colours::Green) {
        if sim.a.abs() < 5 {
            *state = States::Forward;
            return;
        }
        else {
            *state = States::Reverse;
            println!("State: Stop\nsending stop command to MDPS...\n");
            *ignore_input = true;
            return;
        }
    }

    if sim.c.contains(&Colours::Red) && sim.a.abs() < 5 {
        *state = States::MazeDone;
        *continue_maze = false;
        return;
    }

    if sim.c.contains(&Colours::Blue) || sim.c.contains(&Colours::Black) {
        *state = States::Reverse;
        println!("State: Stop\nsending stop command to MDPS...\n");
        *ignore_input = true;
        return
    }
}

fn reverse(sim: &mut NavconSim, state: &mut States) {
    println!("State: Reverse");
    if !(sim.reverse) {
        println!("MARV is reversing...\n");
        sim.reverse = true;
        *state = States::Stop;
        return;
    }
    
    if sim.c.contains(&Colours::Blue) || sim.c.contains(&Colours::Black) {
        if sim.a.abs() >= 45 {
            sim.t_angle = 5;
            if sim.a > 0 {
                *state = States::TurnRight;
                println!("MARV stopped and didn't reverse\n");
                return;
            }
            *state = States::TurnLeft;
            println!("MARV stopped and didn't reverse\n");
            return;
        }
        sim.t_angle = 90 - sim.a;
        *state = States::TurnRight;
        println!("MARV stopped and didn't reverse\n");
        return;
    }

    if sim.a.abs() >= 45 {
        sim.t_angle = 5;
    }
    else {
        sim.t_angle = sim.a;
    }

    if sim.a > 0 {
        *state = States::TurnLeft;
    }
    else {
        *state = States::TurnRight;
        if sim.t_angle != 5 {
            sim.t_angle *= -1;
        }
    }

    println!("MARV stopped and didn't reverse\n");
    
}

fn turn_right(sim: &mut NavconSim, state: &mut States) {
    println!("State: Turn Right");
    sim.reverse = false;
    *state = States::Stop;
    println!("MARV turning right! | {} degrees\n", sim.t_angle);
}

fn turn_left(sim: &mut NavconSim, state: &mut States) {
    println!("State: Turn Left");
    sim.reverse = false;
    *state = States::Stop;
    println!("MARV turning left! | {} degrees\n", sim.t_angle);
}

fn init_test_data(test_d : &mut Vec<TestData>) {
    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Blue, Colours::White, Colours::White, Colours::White, Colours::White], 0, 0.0, 0.0));
    
    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Blue, Colours::White, Colours::White, Colours::White, Colours::White], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));
    
    test_d.push(TestData::new([Colours::Blue, Colours::White, Colours::White, Colours::White, Colours::White], 30, 0.0, 0.0));
    
    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], 30, 0.0, 0.0));
    
    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Blue, Colours::White, Colours::White, Colours::White, Colours::White], 30, 0.0, 0.0));
    
    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], 60, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], 55, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], 50, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], 45, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], 40, 0.0, 0.0));
    
    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], -60, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], -55, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], -50, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], -45, 0.0, 0.0));

    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));

    test_d.push(TestData::new([Colours::Green, Colours::White, Colours::White, Colours::White, Colours::White], -40, 0.0, 0.0));
}