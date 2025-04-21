use std::ptr::null_mut;
use sdl3_sys::render::{SDL_DestroyTexture, SDL_Texture};

pub(crate) struct TextureHnd {
  texture: *mut SDL_Texture
}

impl TextureHnd {
  pub(crate) const EMPTY: TextureHnd = TextureHnd { texture: null_mut() };

  pub(in crate::renderer) unsafe fn new(texture: *mut SDL_Texture) -> Self { Self { texture } }

  pub(in crate::renderer) unsafe fn get_ptr(&self) -> *mut SDL_Texture { self.texture }
}

impl Default for TextureHnd {
  fn default() -> Self { TextureHnd::EMPTY }
}

impl Drop for TextureHnd {
  fn drop(&mut self) {
    if !self.texture.is_null() {
      unsafe { SDL_DestroyTexture(self.texture); }
    }
  }
}