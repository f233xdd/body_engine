use engine::Engine;
use engine::vector;
use engine::{G, Planet};

#[test]
fn main() {
    use std::time;
    let start = time::Instant::now();
    let l = [
        Planet::new(1e10, vector![-5.0, 0.0], vector![0.0, 0.0]),
        Planet::new(1e10, vector![5.0, 0.0], vector![0.0, 0.0]),
    ];
    let mut total: f64 = 0.0;
    let mut eng = Engine::new(l);
    println!("=====start sitmulating=====");
    let mut i: i128 = 0;
    while !(eng.planets()[0].r().x() >= eng.planets()[1].r().x()) {
        i += 1;
        eng.flush(1e-9);
        // println!("{} || {}", eng.planets()[0], eng.planets()[1]);
        total += 1e-9;
    }
    // (f64, 1e-5):
    //     generation_times: 3040163
    //     elapsed_time: 5.3000112s
    //     crash_time: 30.401629999018233s

    // (f128, 1e-9):
    //     generation_times: 
    //     crash_time: 30.401623757000016s

    // 30.40162377244554s （f64, theroy)
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
