# Body Simulator

this is a body simulator written in Rust

## Quick-Start

the following block shows a demo of simulating Sun and Earth system

```rust

use engine::{Engine, G, Planet, vector};

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

loop {
    println!("{eng}");
    eng.flush(1e-3);
}

```