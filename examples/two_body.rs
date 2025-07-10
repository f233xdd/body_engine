use engine::{Engine, G, Planet, vector};

fn main() {
    use std::time;
    let mut eng = Engine::new([
        Planet::new_by_default_name(1e10, vector![-5.0], vector![0.0]),
        Planet::new_by_default_name(1e10, vector![5.0], vector![0.0]),
    ]);
    let mut total: f64 = 0.0;
    let mut i: i128 = 0;
    println!("=====STRAT SIMULATING=====");

    let start = time::Instant::now();

    while eng.planets()[0].relative_r(&eng.planets()[1]).norm() > 0.001 {
        i += 1;
        eng.flush(1e-7);
        total += 1e-7;
    }

    let to_end = start.elapsed();

    println!("generation_times: {i}");
    println!("elapsed_time: {}s", to_end.as_secs_f64());
    println!("crash_time(simutation): {total}s");
    println!("crash_time(theory): {}s", {
        let μ = 1e10 * 2.0;
        let r: f64 = 10.0;
        let pi = std::f64::consts::PI;
        pi / 2.0 * (r.powi(3) / (2.0 * G * μ)).sqrt()
    });
}
