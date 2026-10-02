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

fn tune_param(
    target: f64,
    kp: &mut f64,
    ki: &mut f64,
    kd: &mut f64,
    parameter: &str,
    history: &mut Vec<Test>,
) {
    let dt = 0.01;
    let duration = 10.0;
    let step = 0.1;
    let max_iterations = 100;
    let tolerance = 1e-3;

    let mut direction = 1.0;
    let mut best_error = f64::INFINITY;
    let mut best_value;

    if parameter == "kp" {
        best_value = *kp;
    } else if parameter == "ki" {
        best_value = *ki;
    } else {
        best_value = *kd;
    }

    for _ in 0..max_iterations {
        let mut pid = Pidctrl::new(*kp, *ki, *kd);
        let mut motor = Motor::new();

        let samples = simulate(
            &mut pid,
            &mut motor,
            target,
            dt,
            duration,
        );

        let metrics = analyze(&samples);
        let error = metrics.steady_err;

        history.push(Test {
            kp: *kp,
            ki: *ki,
            kd: *kd,
            error,
            metrics,
        });

        if error <= tolerance {
            break;
        }

        let current_value = if parameter == "kp" {
            *kp
        } else if parameter == "ki" {
            *ki
        } else {
            *kd
        };

        if error < best_error {
            best_error = error;
            best_value = current_value;

            if parameter == "kp" {
                *kp += direction * step;
            } else if parameter == "ki" {
                *ki += direction * step;
            } else {
                *kd += direction * step;
            }
        } else {
            if parameter == "kp" {
                *kp = best_value;
                direction = -direction;
                *kp += direction * step;
            } else if parameter == "ki" {
                *ki = best_value;
                direction = -direction;
                *ki += direction * step;
            } else {
                *kd = best_value;
                direction = -direction;
                *kd += direction * step;
            }
        }
    }
}

pub fn equalize(target: f64) -> Vec<Test> {
    let mut history = Vec::new();
    let mut kp = 1.0;
    let mut ki = 0.1;
    let mut kd = 0.01;
    tune_param(target, &mut kp, &mut ki, &mut kd, "kp", &mut history);
    tune_param(target, &mut kp, &mut ki, &mut kd, "ki", &mut history);
    tune_param(target, &mut kp, &mut ki, &mut kd, "kd", &mut history);

    history
}