use crate::actor::Actor;
use crate::actor::beato::Beato;
use crate::renderer::Renderer;
use crate::state::{State, StateCmd};

#[derive(Default)]
pub(crate) struct GameState {
  beato: Beato
}

impl State for GameState {
  fn tick(&mut self, deltatime: f32) -> StateCmd {
    self.beato.update(deltatime);
    StateCmd::Continue
  }

  fn load(&mut self, renderer: &mut Renderer) {
    self.beato.load_textures(renderer);
  }

  fn draw(&mut self, renderer: &mut Renderer, deltatime: f32) {
    renderer.clear_colour(0x1F, 0x1F, 0x1F);
    self.beato.draw(renderer, deltatime);
  }
}
