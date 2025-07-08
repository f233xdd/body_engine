use super::planet::{Planet, interact};

pub struct Engine<const D: usize, const N: usize> {
    planets: [Planet<D>; N],
}

impl<const D: usize, const N: usize> Engine<D, N> {
    pub fn new(planets: [Planet<D>; N]) -> Self {
        Self { planets }
    }
    pub fn planets(&self) -> &[Planet<D>; N] {
        &self.planets
    }
    pub fn flush(&mut self, dt: f64) {
        for i in 0..N {
            for j in (i + 1)..N {
                let l = &raw mut self.planets[i];
                let r = &raw mut self.planets[j];
                unsafe {
                    interact(&mut *l, &mut *r, dt);
                }
            }
        }
        for p in self.planets.iter_mut() {
            p.flush_r(dt);
        }
    }
}
