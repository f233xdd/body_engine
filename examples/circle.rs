use engine::{Engine, Planet, vector};

const T: f64 = 1e-3;

fn main() {
    use std::time;
    let mut eng = Engine::new([
        Planet::new(
            1.989e30,
            vector!(0.0, 0.0),
            vector!(0.0, 0.0),
            String::from("Sun"),
        ),
        Planet::new(
            5.965e24,
            vector!(1.471e11, 0.0),
            vector!(0.0, 3.029e4),
            String::from("Earth"),
        ),
    ]);
    let mut t = 0.0;
    println!("{t}\n{eng}");
    println!("=====STRAT SIMULATING=====");
    let start = time::Instant::now();
    while t < 31557600.0 {
        // println!("{t}\n{eng}");
        eng.flush(T);
        t += T;
    }
    println!("duration: {}", start.elapsed().as_secs_f64());
    println!("{t}\n{eng}");
}
