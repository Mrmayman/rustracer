use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use rand_xorshift::XorShiftRng;

use crate::utils::{random_double_range, random_double};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn new_default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn setx(&mut self, x: f64) {
        self.e[0] = x;
    }

    pub fn sety(&mut self, y: f64) {
        self.e[1] = y;
    }

    pub fn setz(&mut self, z: f64) {
        self.e[2] = z;
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let low_value: f64 = 1e-8;
        (self.e[0].abs() < low_value)
            && (self.e[1].abs() < low_value)
            && (self.e[2].abs() < low_value)
    }

    pub fn random(rng: &mut XorShiftRng) -> Vec3 {
        Vec3::new(random_double(rng), random_double(rng), random_double(rng))
    }

    pub fn random_range(min: f64, max: f64, rng: &mut XorShiftRng) -> Vec3 {
        Vec3::new(
            random_double_range(min, max, rng),
            random_double_range(min, max, rng),
            random_double_range(min, max, rng),
        )
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e: [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ],
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = f64::min(dot(-uv, n), 1.0);
    let r_out_perp: Vec3 = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: Vec3 = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
    
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_sphere(rng: &mut XorShiftRng) -> Vec3 {
    loop {
        // Keep creating new vectors until we find one inside unit sphere
        let p = Vec3::random_range(-1.0, 1.0, rng);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector(rng: &mut XorShiftRng) -> Vec3 {
    random_in_unit_sphere(rng).unit_vector()
}

pub fn random_on_hemisphere(normal: Vec3, rng: &mut XorShiftRng) -> Vec3 {
    let on_unit_sphere = random_unit_vector(rng);
    if dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}