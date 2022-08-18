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

}