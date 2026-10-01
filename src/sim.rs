use crate::pid::Pidctrl;

pub struct Motor {
    velocity: f64,
}

impl Motor {
    pub fn new() -> Self {
        Motor {
            velocity: 0.0,
        }
    }

    pub fn update(&mut self, control: f64, dt: f64) {
        self.velocity += control * dt;
    }
}

pub struct Sample {
    pub time: f64,
    pub target: f64,
    pub measurement: f64,
    pub control: f64,
}

pub fn simulate(pid: &mut Pidctrl, motor: &mut Motor, target: f64, dt: f64, duration: f64) -> Vec<Sample> {
    let mut samples = Vec::new();
    let mut time = 0.0;
    while time < duration {
        let measurement = motor.velocity;
        let control = pid.update(target, measurement, dt);
        motor.update(control, dt);
        samples.push(Sample {time, target, measurement, control,});
        time += dt;
    }
    samples
}