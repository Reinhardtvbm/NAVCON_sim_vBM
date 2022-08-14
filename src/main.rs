mod navcon;
pub mod colours;

use navcon::*;
use colours::Colours;

enum Straight {
    Forward, Reverse 
}

impl Straight {
    fn value(&self) -> f32 {
        match self {
            Straight::Forward => 5_f32,
            Straight::Reverse => -5_f32,
        }
    }
}

fn main() {
    let (mut green_red, mut blue_black, mut all_white) = (false, false, true);
    let mut navcon_sim = Navcon::new();
    let mut arc_length = 0_f32;
    let mut maze_done = false;

    while maze_done {
        while (all_white) {
            navcon_sim.set_straight(Straight::Forward.value());
            navcon_sim.wait_for_input();
        
            (green_red, blue_black, all_white) = navcon_sim.interperet_colours();
        }

        while green_red {
            navcon_sim.set_stop();
            navcon_sim.set_straight(Straight::Reverse.value());

            if navcon_sim.incidence > 45 {
                arc_length = navcon_sim.start_left_turn(5.0, 3.0);

                while navcon_sim.distance < arc_length {/* wait */}
            }
            else if navcon_sim.incidence > 5 {
                arc_length = navcon_sim.start_left_turn(navcon_sim.incidence as f32, 3.0);

                while navcon_sim.distance < arc_length {/* wait */}
            }

            navcon_sim.set_stop();
        }

        while blue_black {
            navcon_sim.set_stop();
            navcon_sim.set_straight(Straight::Reverse.value());

            if navcon_sim.incidence > 45 {
                arc_length = navcon_sim.start_left_turn(5.0, 3.0);

                while navcon_sim.distance < arc_length {/* wait */}
            }
            else if navcon_sim.incidence > 5 {
                let turn = (90 - navcon_sim.incidence) as f32; 
                arc_length = navcon_sim.start_left_turn(turn, 3.0);

                while navcon_sim.distance < arc_length {/* wait */}
            }

            navcon_sim.set_stop();
        }
    }
}
