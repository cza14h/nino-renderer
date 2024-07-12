use crate::math::{Vec2, Vec3, Vec4};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
  pub position: Vec3,
  pub normal: Option<Vec3>,
  pub texture: Option<Vec2>,
}

pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub name: Option<String>,
}
