mod navcon;
pub mod colours;


use navcon::*;
use colours::Colours;

fn main() {
    let mut navcon_sim = Navcon::new();

    let mock_colours = [
        [Colours::White; 5],
        [Colours::White; 5],
        [Colours::White; 5],
        [Colours::White; 5],
        [Colours::White; 5],
        [Colours::White; 5],
        [Colours::White; 5],
        [Colours::Black, Colours::Black, Colours::White, Colours::White, Colours::White]
    ];

    let mock_incidence: [u16; 8] = [
        0, 0, 0, 0, 0, 0, 3, 3
    ];

    let i = 0;

    while !(navcon_sim.contains_non_white()) {
        navcon_sim.forward();
        transmit_colours(&mut navcon_sim, mock_colours[i]);
        transmit_incidence(&mut navcon_sim, mock_incidence[i]);
    }

    if navcon_sim.colour_sensor.contains(&Colours::Black) ||  navcon_sim.colour_sensor.contains(&Colours::Blue) {
        while navcon_sim.incidence > 45 {

        }

        while navcon_sim.incidence > 5 {

        }
    }

    if navcon_sim.colour_sensor.contains(&Colours::Green) || navcon_sim.colour_sensor.contains(&Colours::Red) {
        while navcon_sim.incidence > 45 {

        }

        while navcon_sim.incidence > 5 {

        }
    }
}
