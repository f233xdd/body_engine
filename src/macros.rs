#[macro_export]
macro_rules! vector {
    ($($v:expr),+) => {
        $crate::Vector::new([$($v,)+])
    }
}
