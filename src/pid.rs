pub struct Pidctrl {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    prev_err: f64,
}

impl Pidctrl {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Pidctrl {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_err: 0.0,
        }
    }

    pub fn update(&mut self, target: f64, measurement: f64, dt: f64) -> f64 {
        if dt <= 0.0 {return 0.0;}
        let error = target - measurement;
        self.integral += error * dt;
        let derivative = (error - self.prev_err) / dt;
        self.prev_err = error;
        self.kp * error
            + self.ki * self.integral
            + self.kd * derivative
    }
}