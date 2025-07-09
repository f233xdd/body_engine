use crate::Vector;

pub const G: f64 = 6.674E-11;
pub const ACCURACY: f64 = 1e-8;

pub type R<const D: usize> = Vector<D>;
pub type V<const D: usize> = Vector<D>;
pub type F<const D: usize> = Vector<D>;
