use crate::pid::Pidctrl;
use crate::metric::{analyze, Metrics};
use crate::sim::{simulate, Motor};

pub struct Test {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub error: f64,
    pub metrics: Metrics,
}

pub fn equlize(target: f64) -> Vec<Test> {
    let mut history = Vec::new();
    let mut kp = 1.0;
    let mut ki = 0.1;
    let mut kd = 0.01;
    let dt = 0.01;
    let duration = 10.0;
    let step = 0.1;
    let max_iterations = 100;
    let tolerance = 1e-3;

    let mut params = [&mut kp, &mut ki, &mut kd];

    for param in params.iter_mut() {
        let mut direction = 1.0;
        let mut previous_error = f64::INFINITY;
        for _ in 0..max_iterations {
            let mut pid = Pidctrl::new(kp, ki, kd);
            let mut motor = Motor::new();
            let samples = simulate(&mut pid, &mut motor, target, dt, duration);
            let metrics = analyze(&samples);

            let error = metrics.steady_err;

            history.push(Test {
                kp,
                ki,
                kd,
                error,
                metrics,
            });

            if error <= tolerance {
                break;
            }

            if error < previous_error {
                previous_error = error;
            } else {
                direction = -direction;
                previous_error = f64::INFINITY;
            }

            **param += direction * step;
        }
    }
    history
}