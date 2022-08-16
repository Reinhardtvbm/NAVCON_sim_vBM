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

struct NavconSim {
    c: [Colours; 5],    // colour sensor
    d: f32,             // distance
    s: f32,             // speed
    a: i32,              // incidence

    reverse: bool,
    t_angle: i32
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

    let mut i = 0_usize;
    let mut cont = true;

    while cont {
        
        i += 1;
        match s {
            States::Stop => stop(&mut navcon_sim, &mut s),
            States::Forward => forward(&mut navcon_sim, &mut s, &mut cont),
            States::Reverse => reverse(&mut navcon_sim, &mut s),
            States::TurnRight => turn_right(&mut navcon_sim, &mut s),
            States::TurnLeft => turn_left(&mut navcon_sim, &mut s),
            States::MazeDone => println!("Maze done!"),
        }
    }   
}

fn stop(sim: &mut NavconSim, state: &mut States) {
    println!("State: Stop");
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

fn forward(sim: &mut NavconSim, state: &mut States, continue_maze: &mut bool) {
    println!("State: Forward");
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
            return;
        }
    }

    if sim.c.contains(&Colours::Red) && sim.a.abs() < 5 {
        *state = States::MazeDone;
        *continue_maze = false;
    }
}

fn reverse(sim: &mut NavconSim, state: &mut States) {
    println!("State: Reverse");
    if !(sim.reverse) {
        sim.reverse = true;
        *state = States::Stop;
        return;
    }
    
    if sim.c.contains(&Colours::Blue) || sim.c.contains(&Colours::Black) {
        if sim.a.abs() >= 45 {
            sim.t_angle = 5;
            if sim.a > 0 {
                *state = States::TurnRight;
                return;
            }
            *state = States::TurnLeft;
            return;
        }
        sim.t_angle = 90 - sim.a;
        *state = States::TurnRight;
        return;
    }

    if sim.a.abs() >= 45 {
        sim.t_angle = 5;
    }
    else {
        sim.t_angle = sim.a;
    }

    if sim.a.abs() > 0 {
        *state = States::TurnLeft;
    }
    else {
        *state = States::TurnRight;
    }
    
}

fn turn_right(sim: &mut NavconSim, state: &mut States) {
    println!("State: Turn Right");
    sim.reverse = false;
    *state = States::Stop;
    println!("MARV turning right! | {} degrees", sim.t_angle);
}

fn turn_left(sim: &mut NavconSim, state: &mut States) {
    println!("State: Turn Left");
    sim.reverse = false;
    *state = States::Stop;
    println!("MARV turning left! | {} degrees", sim.t_angle);
}

fn init_test_data(test_d : &mut Vec<TestData>) {
    test_d.push(TestData::new([Colours::White; 5], 0, 0.0, 0.0));
}