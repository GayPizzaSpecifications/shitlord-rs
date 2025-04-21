pub(crate) mod colour;
pub(crate) mod flip;
pub(crate) mod blendmode;
pub(crate) mod texturehnd;

use crate::application::AppError;
use crate::maths::vector2::Vec2f;
use crate::renderer::blendmode::BlendMode;
use crate::renderer::colour::Colour;
use crate::renderer::flip::Flip;
use crate::renderer::texturehnd::TextureHnd;
use sdl3_image_sys::image::IMG_LoadTexture;
use sdl3_sys::everything::*;
use std::ffi::{c_float, c_void, CStr, CString};
use std::ptr::null;
use crate::maths::rectangle::{FRect, Rect};

pub(crate) struct Renderer {
  renderer: *mut SDL_Renderer,
}

impl Renderer {
  pub(crate) fn new(window: *mut SDL_Window, width: i32, height: i32, vsync: bool)
  -> Result<Self, AppError> {
    let props = unsafe { SDL_CreateProperties() };
    unsafe {
      SDL_SetPointerProperty(props, SDL_PROP_RENDERER_CREATE_WINDOW_POINTER, window as *mut c_void);
      SDL_SetNumberProperty(props, SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER,
        if vsync { 1 } else { 0 });
    }
    let renderer = unsafe { SDL_CreateRendererWithProperties(props) };
    unsafe {
      SDL_DestroyProperties(props);
      if renderer.is_null() {
        return Err(AppError::Error(format!("SDL_CreateRendererWithProperties failed: {}",
          CStr::from_ptr(SDL_GetError()).to_string_lossy())));
      }
      SDL_SetRenderLogicalPresentation(renderer, width, height,
        SDL_LOGICAL_PRESENTATION_LETTERBOX);
    }

    Ok(Self { renderer })
  }

  pub(crate) fn line(&mut self, from: &Vec2f, to: &Vec2f) {
    unsafe { SDL_RenderLine(self.renderer, from.x, from.y, to.x, to.y); }
  }

  pub(crate) fn fill(&mut self, rect: Rect) {
    let dst = SDL_FRect::from(rect);
    unsafe { SDL_RenderFillRect(self.renderer, &dst); }
  }

  pub(crate) fn copy_fill(&mut self, texture: &TextureHnd) {
    unsafe { SDL_RenderTexture(self.renderer, texture.get_ptr(), null(), null()); }
  }

  pub(crate) fn load_texture(&mut self, path: impl AsRef<str>) -> TextureHnd {
    unsafe {
      let filepath = CString::new(path.as_ref()).unwrap();
      let texture = IMG_LoadTexture(self.renderer, filepath.as_ptr());
      if texture.is_null() {
        eprintln!("Texture load failure: {}", CStr::from_ptr(SDL_GetError()).to_string_lossy());
        return TextureHnd::EMPTY;
      }
      TextureHnd::new(texture)
    }
  }

  pub(crate) fn copy(&mut self, texture: &TextureHnd, dst: FRect, angle: f64, flip: Flip) {
    let dst = SDL_FRect::from(dst);
    let flip = SDL_FlipMode::from(flip);
    unsafe {
      SDL_RenderTextureRotated(self.renderer, texture.get_ptr(), null(), &dst, angle, null(), flip);
    }
  }

  #[allow(dead_code)]
  pub(crate) fn text(&mut self, pos: Vec2f, text: &str) {
    self.text_cstr(pos, &CString::new(text).unwrap())
  }

  pub(crate) fn text_cstr(&mut self, pos: Vec2f, text: &CString) {
    unsafe { SDL_RenderDebugText(self.renderer, pos.x, pos.y, text.as_ptr()) };
  }

  pub(crate) fn clear_colour(&mut self, r: u8, g: u8, b: u8) {
    let old = self.get_draw_colour();
    self.set_draw_colour(Colour::rgb(r, g, b));
    self.clear();
    self.set_draw_colour(old);
  }

  pub(crate) fn clear(&mut self) {
    unsafe { SDL_RenderClear(self.renderer) };
  }

  pub(crate) fn get_draw_colour(&self) -> Colour {
    let (mut r, mut g, mut b, mut a) = (0u8, 0u8, 0u8, 0u8);
    unsafe { SDL_GetRenderDrawColor(self.renderer, &mut r, &mut g, &mut b, &mut a) };
    Colour { r, g, b, a }
  }

  pub(crate) fn set_draw_colour(&self, colour: Colour) {
    unsafe { SDL_SetRenderDrawColor(self.renderer, colour.r, colour.g, colour.b, colour.a); }
  }

  pub(crate) fn set_blendmode(&self, mode: BlendMode) {
    unsafe { SDL_SetRenderDrawBlendMode(self.renderer, SDL_BlendMode::from(mode)); }
  }

  pub(crate) fn present(&mut self) {
    unsafe { SDL_RenderPresent(self.renderer); }
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    unsafe { SDL_DestroyRenderer(self.renderer) }
  }
}

impl From<Rect> for SDL_FRect {
  #[inline]
  fn from(rect: Rect) -> Self {
    Self { x: rect.x as c_float, y: rect.y as c_float, w: rect.w as c_float, h: rect.h as c_float }
  }
}

impl From<FRect> for SDL_FRect {
  #[inline]
  fn from(rect: FRect) -> Self {
    Self { x: rect.x, y: rect.y, w: rect.w, h: rect.h }
  }
}
