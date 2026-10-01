mod pid;
mod sim;
mod metric;

use pid::Pidctrl;
use sim::{Motor, simulate};
use metric::{analyze, print_metrics};

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
    let metrics = analyze(&samples);
    print_metrics(&metrics);
}