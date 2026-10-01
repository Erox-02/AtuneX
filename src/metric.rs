use crate::sim::Sample;

pub struct Metrics {
    pub overshoot: f64,
    pub settling_time: f64,
    pub steady_err: f64,
}

pub fn analyze(samples: &[Sample]) -> Metrics {
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

pub fn print_metrics(metrics: &Metrics) {
    println!("overshoot = {:.4}", metrics.overshoot);
    println!("settling_time = {:.4}", metrics.settling_time);
    println!("steady_err = {:.4}", metrics.steady_err);
}