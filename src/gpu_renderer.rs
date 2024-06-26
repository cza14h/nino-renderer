use renderer_macro_derive::renderer;

use crate::{
  camera::Camera,
  image::{ColorAttachment, DepthAttachment},
  math::{self, Barycentric, Vec2, Vec3},
  renderer::*,
  shader::{vertex_rhw_init, Attributes, Shader, Uniforms, Vertex},
  texture::TextureStore,
};

#[rustfmt::skip]
/**
 * for any attr with barycentric interpolate:
 * attr_interpolated = alpha * attr0 + beta * attr1 + gamma * attr2
 * 
 * and with perspective correction [ref: https://www.cs.cornell.edu/courses/cs4620/2015fa/lectures/PerspectiveCorrectZU.pdf]
 * 1/Z = alpha / Z0 + beta / Z1 + gamma / Z2 
 * 
 * hence we can combine barycentric interpolate with perspective correct
 * ==> attr_interpolated / Z = alpha * attr0 / Z0 +  beta * attr1 / Z1 +  gamma * attr2 / Z2
 * ==> attr_interpolated = (alpha * attr0 / Z0 +  beta * attr1 / Z1 +  gamma * attr2 / Z2) * Z
 */
fn perspective_correct_and_barycentric_interpolate(
  z:f32,
  vertices: &[Vertex; 3],
  barycentric: &Barycentric,
) -> Attributes {
  let mut attrs = Attributes::default();

  for index in 0..attrs.float.len() {
    attrs.float[index] = (
      vertices[0].attributes.float[index] * barycentric.alpha() / vertices[0].position.z +
      vertices[1].attributes.float[index] * barycentric.beta()  / vertices[1].position.z +
      vertices[2].attributes.float[index] * barycentric.gamma() / vertices[2].position.z
    ) * z;
    attrs.vec2[index] = (
      vertices[0].attributes.vec2[index] * barycentric.alpha() / vertices[0].position.z +
      vertices[1].attributes.vec2[index] * barycentric.beta()  / vertices[1].position.z +
      vertices[2].attributes.vec2[index] * barycentric.gamma() / vertices[2].position.z
    ) * z;
    attrs.vec3[index] = (
      vertices[0].attributes.vec3[index] * barycentric.alpha() / vertices[0].position.z +
      vertices[1].attributes.vec3[index] * barycentric.beta()  / vertices[1].position.z +
      vertices[2].attributes.vec3[index] * barycentric.gamma() / vertices[2].position.z
    ) * z; 
    attrs.vec4[index] = (
      vertices[0].attributes.vec4[index] * barycentric.alpha() / vertices[0].position.z +
      vertices[1].attributes.vec4[index] * barycentric.beta()  / vertices[1].position.z +
      vertices[2].attributes.vec4[index] * barycentric.gamma() / vertices[2].position.z
    ) * z; 
  }
  attrs
}

#[renderer]
struct Renderer;

impl RendererDraw for Renderer {
  fn draw_triangle(
    &mut self,
    model: &crate::math::Mat4,
    vertices: &[crate::shader::Vertex],
    // count: u32,
    // texture: Option<&crate::texture::Texture>,
    texture_store: &TextureStore,
  ) {
    for i in 0..vertices.len() / 3_usize {
      let index = (i * 3) as usize;
      let mut vertices = [vertices[index], vertices[index + 1], vertices[index + 2]];
      let frustum = self.camera.get_frustum();
      for v in &mut vertices {
        *v = self
          .shader
          .call_vertex_shading(v, &self.uniforms, texture_store);
      }

      for v in &mut vertices {
        v.position = *model * v.position;
      }

      for v in &mut vertices {
        v.position = *self.camera.get_view_matarix() * v.position
      }

      for v in &mut vertices {
        v.position = *frustum.get_mat() * v.position;
      }

      if should_cull(
        &vertices.map(|v| v.position.truncated_to_vec3()),
        // *Vec3::z_axis() * -1_f32,
        self.camera.get_view_direction(),
        self.front_face,
        self.cull,
      ) {
        continue;
      }

      for v in &mut vertices {
        v.position.z = -v.position.w;
      }

      for v in &mut vertices {
        v.position.x /= v.position.w;
        v.position.y /= v.position.w;
      }

      for v in &mut vertices {
        v.position.x =
          (v.position.x + 1.0) * 0.5 * (self.viewport.w as f32 - 1.0) + self.viewport.x as f32;
        v.position.y =
          (v.position.y + 1.0) * 0.5 * (self.viewport.h as f32 - 1.0) + self.viewport.y as f32;
        v.position.w = (v.position.w + 1.0) / 2.0;
      }

      let aabb_min_x = vertices
        .iter()
        .fold(std::f32::MAX, |min, v| {
          if min < v.position.x {
            min
          } else {
            v.position.x
          }
        })
        .ceil()
        .max(0.0);

      let aabb_max_x = vertices
        .iter()
        .fold(std::f32::MIN, |max, v| {
          if max < v.position.x {
            v.position.x
          } else {
            max
          }
        })
        .floor()
        .min(self.color.width() as f32 - 1.0);

      let aabb_min_y = vertices
        .iter()
        .fold(std::f32::MAX, |min, v| {
          if min < v.position.y {
            min
          } else {
            v.position.y
          }
        })
        .ceil()
        .max(0.0);

      let aabb_max_y = vertices
        .iter()
        .fold(std::f32::MIN, |max, v| {
          if max < v.position.y {
            v.position.y
          } else {
            max
          }
        })
        .floor()
        .min(self.color.height() as f32 - 1.0);

      if self.wireframe_mode {
        rasterize_wireframe(
          &vertices,
          &self.shader.fragment_shading,
          &self.uniforms,
          texture_store,
          &mut self.color,
          &mut self.depth,
        );
      }

      for x in (aabb_min_x as u32)..(aabb_max_x as u32) {
        for y in (aabb_min_y as u32)..(aabb_max_y as u32) {
          let barycentric = Barycentric::new(
            &math::Vec2::new(x as f32, y as f32),
            &vertices.map(|v| Vec2::new(v.position.x, v.position.y)),
          );

          if barycentric.is_valid() {
            // let mut color = vertices[0].attributes.vec4[ATTR_COLOR] * barycentric.alpha()
            //   + vertices[1].attributes.vec4[ATTR_COLOR] * barycentric.beta()
            //   + vertices[2].attributes.vec4[ATTR_COLOR] * barycentric.gamma();

            // match texture {
            //   Some(t) => {
            //     let texture_coord = vertices[0].attributes.vec2[ATTR_TEXCOORD]
            //       + barycentric.alpha()
            //       + vertices[1].attributes.vec2[ATTR_TEXCOORD]
            //       + barycentric.beta()
            //       + vertices[2].attributes.vec2[ATTR_TEXCOORD]
            //       + barycentric.gamma();

            //     color *= texture_sample(t, &texture_coord);
            //   }

            //   None => {}
            // }

            let inv_z = barycentric.alpha() / vertices[0].position.z
              + barycentric.beta() / vertices[1].position.z
              + barycentric.gamma() / vertices[2].position.z;
            let z = 1.0 / inv_z;

            if (self.depth.get(x, y) <= z) {
              let attr =
                perspective_correct_and_barycentric_interpolate(z, &vertices, &barycentric);

              let color = self
                .shader
                .call_fragment_shading(&attr, &self.uniforms, texture_store);
              self.color.set(x, y, &color);
              // update the closer depth from the cammer
              self.depth.set(x, y, z);
            }
          }
        }
      }
    }
  }
}
