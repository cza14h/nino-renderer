macro_rules! define_vec {
  ($name:ident, $($p:ident),+) => {
    #[derive(Debug, PartialEq, Copy, Clone, Default)]
    pub struct $name {
      $(
        pub $p:f32,
      )+
    }

    impl $name {
      pub fn new($($p:f32),+) -> Self {
        $name{
          $(
            $p,
          )+

        }
      }

      pub fn zero() -> Self {
        $name{
          $(
            $p:0.0,
          )+
        }
      }

    }
  };
}

define_vec!(Vec2, x, y);
define_vec!(Vec3, x, y, z);
define_vec!(Vec4, x, y, z, w);
