use crate::math::Vec3;
struct TypeBufferAttribute<T: Sized + Copy> {
  data: Vec<T>,
  size: usize,
  normalized: bool,
}

impl<T: Sized + Copy> TypeBufferAttribute<T> {
  pub fn new(data: Vec<T>, size: usize, normalized: bool) -> Self {
    Self {
      data,
      size,
      normalized,
    }
  }
}

// impl Into<Vec3> for Vec<f32> {
//   fn into(self) -> Vec3 {
//     Vec3::new(self[0], self[1], self[2])
//   }
// }

pub trait ExtractRef<T> {
  fn extract(&self) -> Option<T>;
}

impl<T: Sized + Copy> Iterator for TypeBufferAttribute<T> {
  type Item = Vec<T>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut res = vec![];
    let mut count = self.size;
    while count > 0 {
      if let Some(v) = self.data.iter().next() {
        res.push(*v);
      } else {
        return None;
      };

      count -= 1;
    }
    Some(res)
  }
}

macro_rules! typed_array {
  ($enum_name:tt;$($enum:tt-$type:tt-$ty:tt);+) => {
    $(
      pub type $type = TypeBufferAttribute<$ty>;

      impl $type {
        pub fn as_enum(self)-> $enum_name{
          $enum_name::$enum(Box::new(self))
        }

      }


      // impl Extract<Box<$type>> for $enum_name {
      //   fn extract(self)->Option<Box<$type>>{
      //     if let Self::$enum(val) = self {
      //       Some(val)
      //     } else {
      //       None
      //     }
      //   }
      // }
    )+


    impl ExtractRef<Vec3> for  $enum_name {
      fn extract(&self) -> Option<Vec3> {
        let (x,y,z) = match self {
          $(
            Self::$enum(val)=> {
              if val.data.len() < 3{
                return None
              }
              (val.data[0] as f32,val.data[1] as f32,val.data[2] as f32)
            }
          )+
          _ => {
            return None
          }
        };
        Some(Vec3::new(x,y,z))
      }
    }
    pub enum $enum_name {
      $(
       $enum(Box<$type>),
      ) +
    }

  };
}

typed_array!(
  TypeBufferEnum;
  F64-F64BufferAttribute-f64;
  F32-F32BufferAttribute-f32;
  U32-U32BufferAttribute-u32;
  I32-I32BufferAttribute-i32;
  U16-U16BufferAttribute-u16;
  I16-I16BufferAttribute-i16;
  U8-U8BufferAttribute-u8;
  I8-I8BufferAttribute-i8
);

macro_rules! a {
  ($store:ident, $type:ty, $key:tt,!) => {
    crate::core::ExtractRef::<$type>::extract(
      ($store
        .get($key)
        .expect(&format!("error from getting {} from attributes", $key))),
    )
    .expect(&format!(
      "errot from parsing attribute '{}' value to  type '{}'",
      $key,
      stringify!($type)
    ))
  };
}

pub(crate) use a;
