use crate::application::timehelper::TimeHelper;
use crate::fpscalculator::FPSCalculator;
use crate::application::gamepad::GamePad;
use crate::maths::vector2::Vec2f;
use crate::renderer::colour::Colour;
use crate::renderer::Renderer;
use crate::state::splashstate::SplashState;
use crate::state::{State, StateCmd};
use sdl3_sys::everything::*;
use std::ffi::{c_int, CStr, CString};
use std::ptr::addr_of_mut;
use std::ptr::null_mut;

pub(crate) mod gamepad;
mod timehelper;

pub(crate) struct Application {
  window: *mut SDL_Window,
  renderer: Option<Renderer>,
  should_quit: bool,
  state: Option<Box<dyn State>>,
  time: TimeHelper,
  fps_counter: FPSCalculator,
  fps_string: CString,
  currently_fullscreen: bool,
}

impl Application {
  fn new() -> Application {
    Application {
      window: null_mut(),
      renderer: None,
      should_quit: false,
      state: None,
      time: Default::default(),
      fps_counter: Default::default(),
      fps_string: CString::new("").unwrap(),
      currently_fullscreen: false,
    }
  }

  fn change_state<T: State + Default + 'static>(&mut self) {
    if let Some(mut state) = self.state.take() {
      state.quit();
    }
    self.state = Some(Box::new(T::default()));
    if let Some(state) = self.state.as_mut() {
      state.load(self.renderer.as_mut().unwrap());
      state.init();
    }
  }

  fn init(&mut self) -> Result<(), AppError> {
    unsafe {
      if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_GAMEPAD) {
        return Err(AppError::Error(format!("SDL_Init failed: {}",
          CStr::from_ptr(SDL_GetError()).to_string_lossy())));
      }

      let wintitle = CString::new("Find the computer room!").unwrap();
      let winflags = SDL_WINDOW_HIGH_PIXEL_DENSITY | SDL_WINDOW_RESIZABLE;
      self.window = SDL_CreateWindow(wintitle.as_ptr(), 640, 480, winflags);
      if self.window.is_null() {
        return Err(AppError::Error(format!("SDL_CreateWindow failed: {}",
          CStr::from_ptr(SDL_GetError()).to_string_lossy())));
      }
    }

    self.renderer = Some(Renderer::new(self.window, 640, 480, true)?);
    self.change_state::<SplashState>();
    self.time.init();

    Ok(())
  }

  fn event(&mut self, event: SDL_Event) {
    match unsafe { SDL_EventType(event.r#type) } {
      SDL_EVENT_QUIT => self.should_quit = true,
      SDL_EVENT_KEY_DOWN => match unsafe { event.key }.key {
        SDLK_RETURN => unsafe {
          if !event.key.repeat && event.key.r#mod & SDL_KMOD_ALT != 0 {
            SDL_SetWindowFullscreen(self.window, !self.currently_fullscreen);
          }
        }
        SDLK_ESCAPE => self.should_quit = true,
        _ => ()
      }
      SDL_EVENT_WINDOW_ENTER_FULLSCREEN => self.currently_fullscreen = true,
      SDL_EVENT_WINDOW_LEAVE_FULLSCREEN => self.currently_fullscreen = false,
      SDL_EVENT_GAMEPAD_ADDED => unsafe { GamePad::connected_event(event.gdevice.which) }
      SDL_EVENT_GAMEPAD_REMOVED => unsafe { GamePad::removed_event(event.gdevice.which) }
      SDL_EVENT_GAMEPAD_BUTTON_DOWN | SDL_EVENT_GAMEPAD_BUTTON_UP =>
        unsafe { GamePad::button_event(
          event.gbutton.which,
          SDL_GamepadButton(event.gbutton.button as c_int),
          event.gbutton.down
        ) }
      SDL_EVENT_GAMEPAD_AXIS_MOTION =>
        unsafe { GamePad::axis_event(
          event.gaxis.which,
          SDL_GamepadAxis(event.gaxis.axis as c_int),
          event.gaxis.value
        ) }
      _ => ()
    }
  }

  fn draw(&mut self, deltatime: f32) {
    if let Some(ref mut renderer) = self.renderer {
      if let Some(ref mut state) = self.state {
        state.draw(renderer, deltatime);
      }
      renderer.set_draw_colour(Colour::hex(0xFF1F4FFF));
      renderer.text_cstr(Vec2f::ONE * 5.0, &self.fps_string);
      renderer.present()
    }
  }

  fn free(&mut self) {
    if let Some(ref mut state) = self.state {
      state.quit();
    }
    self.state = None;
    self.renderer = None;
    unsafe {
      SDL_DestroyWindow(self.window);
      SDL_Quit();
    }
  }

  pub(crate) fn run() -> u8 {
    unsafe { SDL_SetCurrentThreadPriority(SDL_THREAD_PRIORITY_HIGH) };
    let mut app = Application::new();
    if let Err(err) = app.init() {
      eprintln!("ERROR: {:?}", err);
      app.free();
      return 1;
    }

    let mut exit_code = 0_u8;
    while !app.should_quit {
      app.time.frame_advance();

      // Update FPS metrics
      app.fps_counter.frame(app.time.get_duration(),
        |s| app.fps_string = CString::new(format!("FPS: {}", s)).unwrap());

      // Poll events
      GamePad::advance_frame();
      let mut event: SDL_Event = unsafe { std::mem::zeroed() };
      while unsafe { SDL_PollEvent(addr_of_mut!(event)) } {
        app.event(event);
      }

      // Calculate delta time
      const MIN_DELTA_TIME: f32 = 1.0 / 15.0;
      let delta = f32::min(MIN_DELTA_TIME, app.time.get_deltatime() as f32);

      // Tick and draw
      let cmd = app.state.as_mut().map_or(StateCmd::Continue, |state| state.tick(delta));
      app.draw(delta);

      match cmd {
        // Scene asked us to switch to a different one
        StateCmd::ChangeState(new_state) => {
          if let Some(mut state) = app.state.take() {
            state.quit();
          }
          app.state = Some(new_state);
          if let Some(ref mut new_state) = app.state {
            new_state.load(app.renderer.as_mut().unwrap());
            new_state.init();
          }
        }
        // Scene returned quit
        StateCmd::Quit(code) => {
          exit_code = code;
          app.should_quit = true;
        }
        // No command
        StateCmd::Continue => {}
      }
    }

    app.free();
    return exit_code;
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum AppError {
  Error(String),
  IOError(std::io::Error),
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self { AppError::IOError(err) }
}
