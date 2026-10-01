mod pid;

use pid::Pidctrl;

fn main() {
    let mut pid = Pidctrl::new(1.0, 0.1, 0.01);

    let output = pid.update(100.0, 0.0, 0.01);

    println!("control output: {output}");
}