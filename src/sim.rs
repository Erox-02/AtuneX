pub struct Pidctrl {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    prev_err: f64,
}

impl Pidctrl {
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
        let derivative = (error - self.prev_err) / dt;
        self.prev_err = error;
        self.kp * error
            + self.ki * self.integral
            + self.kd * derivative
    }
}

struct Motor {
    velocity: f64,
}

impl Motor {
    fn new() -> Self {
        Motor {
            velocity: 0.0,
        }
    }

    fn update(&mut self, control: f64, dt: f64) {
        self.velocity += control * dt;
    }
}

struct Sample {
    time: f64,
    target: f64,
    measurement: f64,
    control: f64,
}

struct Metrics {
    overshoot: f64,
    settling_time: f64,
    steady_err: f64,
}

fn simulate(pid: &mut Pidctrl, motor: &mut Motor, target: f64, dt: f64, duration: f64) -> Vec<Sample> {
    let mut samples = Vec::new();
    let mut time = 0.0;

    while time < duration {
        let measurement = motor.velocity;
        let control = pid.update(target, measurement, dt);
        motor.update(control, dt);
        samples.push(Sample {
            time,
            target,
            measurement,
            control,
        });

        time += dt;
    }
    samples
}

fn analyze(samples: &[Sample]) -> Metrics {
    let target = samples[0].target;

    let maximum = samples
        .iter()
        .map(|s| s.measurement)
        .fold(f64::NEG_INFINITY, f64::max);

    let overshoot = maximum - target;
    let final_measurement = samples[samples.len() - 1].measurement;
    let steady_err = (target - final_measurement).abs();
    let tolerance = 0.02 * target.abs();
    let mut settling_time = f64::NAN;

    for (i, sample) in samples.iter().enumerate() {
        let error = (target - sample.measurement).abs();

        if error <= tolerance {
            let remains_inside = samples[i..]
                .iter()
                .all(|s| (target - s.measurement).abs() <= tolerance);

            if remains_inside {
                settling_time = sample.time;
                break;
            }
        }
    }

    Metrics {
        overshoot,
        settling_time,
        steady_err,
    }
}

fn print_metrics(metrics: &Metrics) {
    println!("overshoot: {:.4}", metrics.overshoot);
    println!("settling_time: {:.4}", metrics.settling_time);
    println!("steady_err: {:.4}", metrics.steady_err);
}

fn main() {
    let mut pid = Pidctrl::new(1.0, 0.1, 0.01);
    let mut motor = Motor::new();
    let target = 100.0;
    let dt = 0.01;
    let duration = 10.0;
    let samples = simulate(&mut pid, &mut motor, target, dt, duration);
    let metrics = analyze(&samples);

    print_metrics(&metrics);
}