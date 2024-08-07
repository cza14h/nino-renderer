use super::{Vec3, Vec4};
use std::ops::{Add, Div, Mul};

macro_rules! define_mat {
  ($name:ident, $dim:expr) => {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct $name {
      data: [f32; $dim * $dim],
    }

    impl $name {
      pub fn from_row(data: &[f32; $dim * $dim]) -> $name {
        $name { data: data.clone() }
      }

      pub fn from_col(data: &[f32; $dim * $dim]) -> $name {
        let mut mat = $name::zeros();
        for x in 0..$dim {
          for y in 0..$dim {
            mat.set(x, y, data[y + $dim * x]);
          }
        }
        mat
      }

      pub fn zeros() -> $name {
        $name {
          data: [0.0; $dim * $dim],
        }
      }
      pub fn ones() -> $name {
        $name {
          data: [1.0; $dim * $dim],
        }
      }

      pub fn identity() -> $name {
        let mut mat = $name::zeros();
        for i in 0..$dim {
          mat.set(i, i, 1.0);
        }
        mat
      }

      pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[x + y * $dim]
      }

      pub fn set(&mut self, x: usize, y: usize, value: f32) {
        self.data[x + y * $dim] = value;
      }

      pub fn transpose(&self) -> $name {
        let mut result = $name::identity();
        for x in 0..$dim {
          for y in 0..$dim {
            result.set(y, x, self.get(x, y));
          }
        }
        result
      }
    }
    impl Mul for $name {
      type Output = Self;
      fn mul(self, rhs: Self) -> Self::Output {
        let mut mat = $name::zeros();

        for y in 0..$dim {
          for x in 0..$dim {
            let mut sum = 0.0;
            for d in 0..$dim {
              sum += self.get(d, y) * rhs.get(x, d);
            }
            mat.set(x, y, sum);
          }
        }
        mat
      }
    }

    impl Mul<f32> for $name {
      type Output = Self;
      fn mul(self, rhs: f32) -> Self::Output {
        let mut mat = $name::zeros();
        for x in 0..$dim {
          for y in 0..$dim {
            mat.set(x, y, self.get(x, y) * rhs);
          }
        }
        mat
      }
    }

    impl Div<f32> for $name {
      type Output = Self;
      fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
      }
    }

    impl Add for $name {
      type Output = Self;
      fn add(self, rhs: Self) -> Self::Output {
        let mut mat = $name::zeros();
        for y in 0..$dim {
          for x in 0..$dim {
            let sum = self.get(x, y) + rhs.get(x, y);
            mat.set(x, y, sum)
          }
        }
        mat
      }
    }

    impl PartialEq for $name {
      fn eq(&self, other: &Self) -> bool {
        self.data == other.data
      }
    }
  };
}

define_mat!(Mat2, 2);
define_mat!(Mat3, 3);
define_mat!(Mat4, 4);

impl Mul<Vec4> for Mat4 {
  type Output = Vec4;

  fn mul(self, rhs: Vec4) -> Self::Output {
    Vec4::new(
      self.get(0, 0) * rhs.x
        + self.get(1, 0) * rhs.y
        + self.get(2, 0) * rhs.z
        + self.get(3, 0) * rhs.w,
      self.get(0, 1) * rhs.x
        + self.get(1, 1) * rhs.y
        + self.get(2, 1) * rhs.z
        + self.get(3, 1) * rhs.w,
      self.get(0, 2) * rhs.x
        + self.get(1, 2) * rhs.y
        + self.get(2, 2) * rhs.z
        + self.get(3, 2) * rhs.w,
      self.get(0, 3) * rhs.x
        + self.get(1, 3) * rhs.y
        + self.get(2, 3) * rhs.z
        + self.get(3, 3) * rhs.w,
    )
  }
}

#[rustfmt::skip]
pub fn apply_translate(offset: &Vec3) -> Mat4 {
  Mat4::from_row(&[
    1.0, 0.0, 0.0, offset.x,
    0.0, 1.0, 0.0, offset.y,
    0.0, 0.0, 1.0, offset.z,
    0.0, 0.0, 0.0, 1.0,
  ])
}

#[rustfmt::skip]
pub fn apply_eular_rotate_y(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();
  Mat4::from_row(&[
      c, 0.0,   s, 0.0,
    0.0, 1.0, 0.0, 0.0,
     -s, 0.0,   c, 0.0,
    0.0, 0.0, 0.0, 1.0,
  ])
}

#[rustfmt::skip]
pub fn apply_eular_rotate_x(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();
  Mat4::from_row(&[
    1.0, 0.0, 0.0, 0.0,
    0.0,   c,  -s, 0.0,
    0.0,   s,   c, 0.0,
    0.0, 0.0, 0.0, 1.0,
  ])
}

#[rustfmt::skip]
pub fn apply_eular_rotate_z(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();
  Mat4::from_row(&[
      c,  -s, 0.0, 0.0,
      s,   c, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
  ])
}

pub fn apply_eular_rotate_xyz(rotation: &Vec3) -> Mat4 {
  apply_eular_rotate_z(rotation.z)
    * apply_eular_rotate_y(rotation.y)
    * apply_eular_rotate_x(rotation.x)
}
