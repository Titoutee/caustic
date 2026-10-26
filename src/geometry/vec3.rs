use std::{
    intrinsics::sqrtf64,
    ops::{Add, Mul, Not, Sub},
};

pub trait MulS<T: Mul> {
    type Output;

    fn muls(self, s: T) -> Self::Output;
}

pub trait Dot {
    type Output;

    fn dot(self, rhs: Self) -> Self::Output;
}

pub trait Cross {
    type Output;

    fn cross(self, rhs: Self) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3(f64, f64, f64);

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Vec3(a, b, c) = self;
        let Vec3(d, e, f) = rhs;
        Vec3(a + d, b + e, c + f)
    }
}

impl Not for Vec3 {
    type Output = Self;
    fn not(self) -> Self::Output {
        let Vec3(a, b, c) = self;
        Vec3(-a, -b, -c)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + !rhs
    }
}

impl MulS<f64> for Vec3 {
    type Output = Self;

    fn muls(self, s: f64) -> Self::Output {
        let Vec3(a, b, c) = self;
        Vec3(a * s, b * s, c * s)
    }
}

impl Dot for Vec3 {
    type Output = f64;

    fn dot(self, rhs: Self) -> Self::Output {
        let Vec3(a, b, c) = self;
        let Vec3(d, e, f) = rhs;

        a * d + b * e + c * f
    }
}

impl Cross for Vec3 {
    type Output = Self;

    fn cross(self, rhs: Self) -> Self::Output {
        let Vec3(a, b, c) = self;
        let Vec3(d, e, f) = rhs;

        Vec3(b * f - c * e, c * d - a * f, a * e - b * d)
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        sqrtf64(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit(&self) -> Self {
        self.muls(1. / self.length())
    }

    pub fn coords(&self) -> (f64, f64, f64) {
        let Vec3(x, y, z) = *self;
        (x, y, z)
    }
}
