pub struct Pidctrl {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    prev_err: f64,
}

pub impl Pidctrl {
    fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Pidctrl {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_err: 0.0,
        }
    }

    fn update(&mut self, target: f64, measurement: f64, dt: f64) -> f64 {
        if dt <= 0.0 {return 0.0;}
        let error = target - measurement;
        self.integral += error * dt;
        let derivative = (error - self.prev_err) / dt ;
        self.prev_err = error;
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}

struct Motor {
    velocity: f64,
}

impl Motor {
    fn new() -> Self {
        Motor { velocity: 0.0 }
    }

    fn update(&mut self, control: f64, dt: f64) {
        self.velocity += control * dt;
    }
}

fn main() {
    let mut pid = Pidctrl::new(1.0, 0.1, 0.01);
    let mut motor = Motor::new();
    let target = 100.0;
    let mut time = 0.0;
    let dt = 0.01;

    while time < 10.0 {
        let measurement = motor.velocity;
        let control = pid.update(target, measurement, dt);
        motor.update(control, dt);
        println!(
            "time: {:.2}, target: {:.2}, measurement: {:.2}, control: {:.2}",
            time, target, measurement, control
        );
        time += dt;
    }
}