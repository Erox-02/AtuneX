mod pid;
mod sim;
mod metric;
mod eq;

use eq::equalize;

fn main() {
    let target = 100.0;
    let history = equalize(target);
    println!("tests = {}", history.len());
    for (i, test) in history.iter().enumerate() {
        println!(
            "test {} | kp = {:.4}, ki = {:.4}, kd = {:.4}, error = {:.6}",
            i + 1,
            test.kp,
            test.ki,
            test.kd,
            test.error
        );
    }

    if let Some(best) = history
        .iter()
        .min_by(|a, b| a.error.partial_cmp(&b.error).unwrap())
    {
        println!("\nbest:");
        println!("kp = {:.4}", best.kp);
        println!("ki = {:.4}", best.ki);
        println!("kd = {:.4}", best.kd);
        println!("error = {:.6}", best.error);
    }
}