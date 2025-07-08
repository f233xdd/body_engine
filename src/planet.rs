use std::fmt;

use super::vector::Vector;

pub const G: f64 = 6.674E-11;

#[derive(Clone)]
pub struct Planet<const D: usize> {
    m: f64,
    r: Vector<D>,
    v: Vector<D>,
}

impl<const D: usize> Planet<D> {
    pub fn new(m: f64, r: Vector<D>, v: Vector<D>) -> Self {
        Self { m, r, v }
    }
    pub fn m(&self) -> &f64 {
        &self.m
    }
    pub fn r(&self) -> &Vector<D> {
        &self.r
    }
    pub fn force(&mut self, f: &Vector<D>, dt: f64) {
        self.v.add_to(&(f / *self.m() * dt));
    }
    pub fn flush_r(&mut self, dt: f64) {
        let dr = &self.v * dt;
        self.r.add_to(&dr);
    }
}

impl<const D: usize> fmt::Display for Planet<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r: {}, v: {}", self.r, self.v)
    }
}

pub fn interact<const D: usize>(p1: &mut Planet<D>, p2: &mut Planet<D>, dt: f64) {
    let dr = p1.r() - p2.r();
    let gravity = &dr * (G * p1.m() * p2.m() / &dr.norm().powi(3));
    p2.force(&gravity, dt);
    p1.force(&(-gravity), dt);
}
