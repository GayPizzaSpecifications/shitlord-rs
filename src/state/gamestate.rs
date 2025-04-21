use crate::actor::beato::Beato;
use crate::actor::Actor;
use crate::gamepad::button::{PadButton, PadButtons};
use crate::gamepad::GamePad;
use crate::renderer::Renderer;
use crate::state::beatoburnerstate::BeatoBurnerState;
use crate::state::{State, StateCmd};

#[derive(Default)]
pub(crate) struct GameState {
  beato: Beato,
  sequence_index: i32,
}

impl State for GameState {
  fn tick(&mut self, deltatime: f32) -> StateCmd {
    self.beato.update(deltatime);
    if let Some(pad) = GamePad::current() {
      if pad.pressed_any(PadButtons::from(PadButton::DPadLeft) | PadButton::DPadRight.into()
          | PadButton::DPadUp.into() | PadButton::DPadDown.into() | PadButton::East.into()
          | PadButton::South.into() | PadButton::Start.into()
      ) {
        match self.sequence_index {
          0 | 1 if { pad.pressed(PadButton::DPadUp) } => self.sequence_index += 1,
          2 | 3 if { pad.pressed(PadButton::DPadDown) } => self.sequence_index += 1,
          4 | 6 if { pad.pressed(PadButton::DPadLeft) } => self.sequence_index += 1,
          5 | 7 if { pad.pressed(PadButton::DPadRight) } => self.sequence_index += 1,
          8 if { pad.pressed(PadButton::East) } => self.sequence_index += 1,
          9 if { pad.pressed(PadButton::South) } => self.sequence_index += 1,
          10 if { pad.pressed(PadButton::Start) } => {
            return StateCmd::ChangeState(Box::new(BeatoBurnerState::default()));
            }
          _ => self.sequence_index = 0,
        }
      }
    }
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
