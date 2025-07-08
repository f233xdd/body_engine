use std::usize;

pub const ACCURACY: f64 = 1e-8;

pub fn approximate(v1: f64, v2: f64) -> bool {
    (v1 - v2).abs() <= ACCURACY
}

#[derive(Debug, Clone)]
pub struct Vector<const L: usize> {
    data: [f64; L],
}

impl<const L: usize> Vector<L> {
    pub fn new(data: [f64; L]) -> Self {
        Self { data }
    }
    pub fn norm(&self) -> f64 {
        self.data.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        Self {
            data: self
                .data
                .iter()
                .map(|x| x / norm)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
    pub fn move_to(&mut self, destination: &Self) {
        for i in 0..L {
            self.data[i] = destination.data[i];
        }
    }
    pub fn add_to(&mut self, other: &Self) {
        for i in 0..L {
            self.data[i] += other.data[i];
        }
    }
}

pub type PlaneVec = Vector<2>;
pub type SpaceVec = Vector<3>;

impl PlaneVec {
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    pub fn cross(&self, rhs: &Self) -> SpaceVec {
        SpaceVec {
            data: [0.0, 0.0, self.x() * rhs.y() - self.y() * rhs.x()],
        }
    }
}

impl SpaceVec {
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    pub fn z(&self) -> f64 {
        self.data[2]
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        SpaceVec {
            data: [
                self.y() * rhs.z() - self.z() * rhs.y(),
                -self.x() * rhs.z() + self.z() * rhs.x(),
                self.x() * rhs.y() - self.y() * rhs.x(),
            ],
        }
    }
}

impl<const L: usize> std::ops::Add for Vector<L> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Add for &Vector<L> {
    type Output = Vector<L>;

    fn add(self, other: Self) -> Vector<L> {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Sub for Vector<L> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Sub for &Vector<L> {
    type Output = Vector<L>;

    fn sub(self, other: Self) -> Vector<L> {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Mul<f64> for Vector<L> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|x| x * scalar)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Mul<f64> for &Vector<L> {
    type Output = Vector<L>;

    fn mul(self, scalar: f64) -> Vector<L> {
        Vector {
            data: self
                .data
                .iter()
                .map(|x| x * scalar)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Mul<Vector<L>> for Vector<L> {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<const L: usize> std::ops::Mul<&Vector<L>> for &Vector<L> {
    type Output = f64;

    fn mul(self, other: &Vector<L>) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<const L: usize> std::ops::Div<f64> for Vector<L> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|x| x / scalar)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Div<f64> for &Vector<L> {
    type Output = Vector<L>;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            data: self
                .data
                .iter()
                .map(|x| x / rhs)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Neg for Vector<L> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            data: self
                .data
                .iter()
                .map(|x| -x)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> std::ops::Neg for &Vector<L> {
    type Output = Vector<L>;

    fn neg(self) -> Self::Output {
        Vector {
            data: self
                .data
                .iter()
                .map(|x| -x)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const L: usize> PartialEq for Vector<L> {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(a, b)| approximate(*a, *b))
    }
}

impl<const L: usize> std::fmt::Display for Vector<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tmp = self
            .data
            .iter()
            .map(|i| format!("{i}, "))
            .collect::<String>();
        tmp.pop();
        tmp.pop();
        write!(f, "{}", ["(", tmp.as_str(), ")"].concat())
    }
}
