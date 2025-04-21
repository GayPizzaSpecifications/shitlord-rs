use crate::actor::Actor;
use crate::actor::beato::Beato;
use crate::application::gamepad::button::PadButton;
use crate::application::gamepad::GamePad;
use crate::maths::rectangle::FRect;
use crate::maths::vector2::Vec2f;
use crate::random::drand48::Drand48;
use crate::random::java::JavaExtensions;
use crate::renderer::colour::Colour;
use crate::renderer::flip::Flip;
use crate::renderer::Renderer;
use crate::renderer::texturehnd::TextureHnd;
use crate::state::{State, StateCmd};

#[allow(dead_code)]
#[derive(Default)]
pub(crate) struct BeatoBurnerState {
  beato: Beato,
  bgpad: TextureHnd,
  bgbad: TextureHnd,
  bad: bool,
  shakeshake: f32,
  random: Drand48,
}

impl State for BeatoBurnerState {
  fn init(&mut self) {
    self.beato.set_position(Vec2f::ONE * -20.0)
  }

  fn load(&mut self, renderer: &mut Renderer) {
    self.bgpad = renderer.load_texture("gamepad.jpeg");
    self.bgbad = renderer.load_texture("gamebad.jpg");
    self.beato.load_textures(renderer);
  }

  fn tick(&mut self, deltatime: f32) -> StateCmd {
    if self.shakeshake > 0.0 {
      self.shakeshake = f32::max(0.0, self.shakeshake - deltatime);
    }
    self.beato.update(deltatime);
    if let Some(pad) = GamePad::current() {
      if pad.pressed(PadButton::East) {
        if !self.bad {
          self.shakeshake = 1.0;
        }
        self.bad = true;
      }
    }
    StateCmd::Continue
  }

  fn draw(&mut self, renderer: &mut Renderer, deltatime: f32) {
    renderer.set_draw_colour(Colour::RED);
    renderer.clear();
    let bgtex = if !self.bad { &self.bgpad } else { &self.bgbad };
    if self.shakeshake > f32::EPSILON {
      let shakevec = Vec2f::new(
        self.random.next_range(-32..32) as f32,
        self.random.next_range(-32..32) as f32) * self.shakeshake;
      renderer.copy(bgtex, FRect::new(shakevec.x, shakevec.y, 640.0, 480.0), 0.0, Flip::None);
    } else { renderer.copy_fill(bgtex); }
    self.beato.draw(renderer, deltatime);
  }
}
