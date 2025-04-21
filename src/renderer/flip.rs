use sdl3_sys::surface::*;

#[allow(dead_code)]
pub(crate) enum Flip {
  None,
  Horizontal,
  Vertical,
  Diagonal
}

impl From<Flip> for SDL_FlipMode {
  fn from(mode: Flip) -> Self {
    match mode {
      Flip::None => SDL_FLIP_NONE,
      Flip::Horizontal => SDL_FLIP_HORIZONTAL,
      Flip::Vertical => SDL_FLIP_VERTICAL,
      Flip::Diagonal => SDL_FlipMode(SDL_FLIP_HORIZONTAL.0 | SDL_FLIP_VERTICAL.0)
    }
  }
}
