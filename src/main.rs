mod pid;
mod sim;

use pid::Pidctrl;
use sim::{Motor, simulate};

fn main() {
    let mut pid = Pidctrl::new(1.0, 0.1, 0.01);
    let mut motor = Motor::new();

    let samples = simulate(
        &mut pid,
        &mut motor,
        100.0,
        0.01,
        10.0,
    );

    println!("samples = {}", samples.len());
}