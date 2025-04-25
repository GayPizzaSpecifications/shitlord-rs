use sdl3_sys::scancode::SDL_Scancode;

#[allow(dead_code)]
pub(crate) enum Key {
  Up,
  Down,
  Left,
  Right,
  Space
}

impl From<Key> for SDL_Scancode {
  fn from(key: Key) -> Self {
    match key {
      Key::Up =>    SDL_Scancode::UP,
      Key::Down =>  SDL_Scancode::DOWN,
      Key::Left =>  SDL_Scancode::LEFT,
      Key::Right => SDL_Scancode::RIGHT,
      Key::Space => SDL_Scancode::SPACE
    }
  }
}
