use crate::maths::rectangle::Rect;
use crate::renderer::{Renderer, colour::Colour};
use crate::renderer::blendmode::BlendMode;
use crate::renderer::texturehnd::TextureHnd;
use crate::state::gamestate::GameState;
use crate::state::{State, StateCmd};

#[derive(Default)]
pub(crate) struct SplashState {
  time: f32, fade: f32,
  bgtex: TextureHnd
}

impl State for SplashState {
  fn init(&mut self) {
    self.time = 0.0;
    self.fade = 1.0;
  }

  fn quit(&mut self) {}

  fn tick(&mut self, deltatime: f32) -> StateCmd {
    self.time += deltatime;
    if self.time < 2.0 {
      self.fade = (self.fade - 0.75 * deltatime).max(0.0);
    } else if self.time < 3.2 {
      self.fade = (self.fade + 0.85 * deltatime).min(1.0);
    } else {
      return StateCmd::ChangeState(Box::new(GameState::default()))
    }
    StateCmd::Continue
  }

  fn load(&mut self, renderer: &mut Renderer) {
    self.bgtex = renderer.load_texture("gamepad.jpeg");
  }

  fn draw(&mut self, renderer: &mut Renderer, _deltatime: f32) {
    renderer.set_draw_colour(Colour::BLACK);
    renderer.clear();
    renderer.copy_fill(&self.bgtex);
    renderer.set_blendmode(BlendMode::Blend);
    let alpha = (self.fade * 255.0) as u8;
    renderer.set_draw_colour(Colour::rgba(0x00, 0x00, 0x00, alpha));
    renderer.fill(Rect::new(0, 0, 640, 480));
  }
}
