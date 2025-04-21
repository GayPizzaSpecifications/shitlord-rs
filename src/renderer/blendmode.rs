use sdl3_sys::blendmode::*;

#[allow(dead_code)]
pub(crate) enum BlendMode {
  None,
  Blend,
  Add,
  AddPremultiplied,
  Modulate,
  Multiply,
  Invalid
}

impl From<BlendMode> for SDL_BlendMode {
  fn from(value: BlendMode) -> Self {
    match value {
      BlendMode::None => SDL_BLENDMODE_NONE,
      BlendMode::Blend => SDL_BLENDMODE_BLEND,
      BlendMode::Add => SDL_BLENDMODE_ADD,
      BlendMode::AddPremultiplied => SDL_BLENDMODE_ADD_PREMULTIPLIED,
      BlendMode::Modulate => SDL_BLENDMODE_MOD,
      BlendMode::Multiply => SDL_BLENDMODE_MUL,
      _ => SDL_BLENDMODE_INVALID
    }
  }
}
