#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Mat33 {
    row0: Vec3,
    row1: Vec3,
    row2: Vec3,
}

impl Vec3 {
    pub const fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(vx: f64, vy: f64, vz: f64) -> Self {
        Vec3 {
            x: 0.0 + vx,
            y: 0.0 + vy,
            z: 0.0 + vz,
        }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn norm(self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn unit(self) -> Self {
        let scale = 1.0 / self.norm();
        if scale == 1.0 {
            self
        } else {
            self * scale
        }
    }

    pub fn square_distance(self, other: Self) -> f64 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.x();
        let dz = self.z() - other.z();
        dx * dx + dy * dy + dz * dz
    }
}

impl ::std::convert::From<[f64; 3]> for Vec3 {
    fn from(arr: [f64; 3]) -> Self {
        Vec3::new(arr[0], arr[1], arr[2])
    }
}

impl ::std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ::std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ::std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ::std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Mat33 {
    pub fn new(r0: Vec3, r1: Vec3, r2: Vec3) -> Mat33 {
        Mat33 {
            row0: r0,
            row1: r1,
            row2: r2,
        }
    }

    pub fn row0(self) -> Vec3 {
        self.row0
    }

    pub fn row1(self) -> Vec3 {
        self.row1
    }

    pub fn row2(self) -> Vec3 {
        self.row2
    }

    pub fn transpose(self) -> Self {
        let r0 = self.row0();
        let r1 = self.row1();
        let r2 = self.row2();
        Mat33::new(
            Vec3::new(r0.x(), r1.x(), r2.x()),
            Vec3::new(r0.y(), r1.y(), r2.y()),
            Vec3::new(r0.z(), r1.z(), r2.z()),
        )
    }

    pub fn multm(self, other: Self) -> Self {
        let t2 = other.transpose();
        let m1r0 = self.row0();
        let m1r1 = self.row1();
        let m1r2 = self.row2();

        let t2r0 = t2.row0();
        let t2r1 = t2.row1();
        let t2r2 = t2.row2();

        Mat33::new(
            Vec3::new(m1r0.dot(t2r0), m1r0.dot(t2r1), m1r0.dot(t2r2)),
            Vec3::new(m1r1.dot(t2r0), m1r1.dot(t2r1), m1r1.dot(t2r2)),
            Vec3::new(m1r2.dot(t2r0), m1r2.dot(t2r1), m1r2.dot(t2r2)),
        )
    }

    pub fn multv(self, vec: Vec3) -> Vec3 {
        Vec3::new(
            vec.dot(self.row0()),
            vec.dot(self.row1()),
            vec.dot(self.row2()),
        )
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn add_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3::new(5.0, 7.0, 9.0), v1 + v2);
    }

    #[test]
    fn multiply_vec3_by_f64() {
        let v = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3::new(8.0, 10.0, 12.0), v * 2.0);
    }
}
