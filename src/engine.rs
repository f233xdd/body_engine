use std::fmt;

use super::{F, G, Planet};

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

impl<const D: usize, const N: usize> fmt::Display for Engine<D, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in self.planets() {
            writeln!(f, "{p}")?;
        }
        Ok(())
    }
}

/// F = G * m1 * m2 / r ^ 2
pub fn gravity<const D: usize>(p1: &mut Planet<D>, p2: &mut Planet<D>) -> F<D> {
    let dr = p2.relative_r(p1);
    &dr * (G * p1.m() * p2.m() / &dr.norm().powi(3))
}

pub fn interact<const D: usize>(p1: &mut Planet<D>, p2: &mut Planet<D>, dt: f64) {
    let g = gravity(p1, p2);
    p2.force(&g, dt);
    p1.force(&(-g), dt);
}
