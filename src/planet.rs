use std::fmt;

use super::{F, R, V};

#[derive(Clone)]
pub struct Planet<const D: usize> {
    m: f64,
    r: R<D>,
    v: V<D>,
}

impl<const D: usize> Planet<D> {
    pub fn new(m: f64, r: R<D>, v: V<D>) -> Self {
        Self { m, r, v }
    }
    pub fn m(&self) -> &f64 {
        &self.m
    }
    pub fn r(&self) -> &R<D> {
        &self.r
    }
    pub fn v(&self) -> &V<D> {
        &self.v
    }
    pub fn force(&mut self, f: &F<D>, dt: f64) {
        self.v.add_to(&(f / *self.m() * dt));
    }
    pub fn flush_r(&mut self, dt: f64) {
        let dr = &self.v * dt;
        self.r.add_to(&dr);
    }
    pub fn is_crashed(&self, other: &Planet<D>) -> bool {
        self.r() == other.r()
    }
    /// self -> other
    pub fn relative_r(&self, other: &Planet<D>) -> R<D> {
        other.r() - self.r()
    }
    /// self -> other
    pub fn relative_v(&self, other: &Planet<D>) -> V<D> {
        other.v() - self.v()
    }
}

impl<const D: usize> fmt::Display for Planet<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r: {}, v: {}", self.r, self.v)
    }
}
