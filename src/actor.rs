use crate::maths::vector2::Vec2f;
use crate::renderer::Renderer;

pub(crate) mod beato;

#[allow(dead_code)]
pub(crate) trait Actor {
  fn get_position(&self) -> Vec2f;
  fn set_position(&mut self, new_position: Vec2f);

  fn update(&mut self, deltatime: f32);
  fn draw(&mut self, renderer: &mut Renderer, deltatime: f32);
}
