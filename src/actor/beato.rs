use crate::actor::Actor;
use crate::gamepad::deadzone::DeadZone2D;
use crate::gamepad::{GamePad, button::PadButton};
use crate::maths::extent::Extent;
use crate::maths::rectangle::FRect;
use crate::maths::vector2::{Vec2f, Vector2};
use crate::renderer::colour::Colour;
use crate::renderer::flip::Flip;
use crate::renderer::Renderer;
use crate::renderer::texturehnd::TextureHnd;
use crate::random::drand48::Drand48;
use crate::random::java::JavaExtensions;

pub(crate) struct Beato {
  pos: Vector2<f32>,
  vel: Vector2<f32>,
  flip: bool,
  lazervec: Vector2<f32>,
  lazering: bool,
  guy_texture: TextureHnd,
  random: Drand48,
}

impl Default for Beato {
  fn default() -> Self {
    Self {
      pos: Vector2::ZERO,
      vel: Vector2::ZERO,
      flip: false,
      lazervec: Vector2::ZERO,
      lazering: false,
      guy_texture: TextureHnd::EMPTY,
      random: Default::default(),
    }
  }
}

impl Beato {
  pub(crate) fn load_textures(&mut self, renderer: &mut Renderer) {
    self.guy_texture = renderer.load_texture("beato.png");
  }

  fn lightning(&mut self, renderer: &mut Renderer, v1: Vec2f, angle: f32, levels: i32) {
    const ADVANCE: f32 = 10.0;
    const JITTER: f32 = 8.0;
    const ANGLE_JITTER: f32 = 0.18;

    let v2 = v1 + Vec2f::new(angle.cos(), angle.sin()) * ADVANCE + Vec2f::new(
      self.random.next_float() * JITTER - JITTER * 0.5,
      self.random.next_float() * JITTER - JITTER * 0.5);

    let bright = self.random.next_bound(0x7F);
    let colour = Colour::rgb(
      (0x7F + bright) as u8,
      (0x7F + bright) as u8,
      0xFF);
    renderer.set_draw_colour(colour);
    renderer.line(&v1, &v2);

    let v3 = Vec2f::new(
      (v2.x + 32.0).rem_euclid(640.0 + 64.0) - 32.0,
      (v2.y + 48.0).rem_euclid(480.0 + 96.0) - 48.0
    );

    if levels > 1 {
      let rando = angle + self.random.next_float() * ANGLE_JITTER - ANGLE_JITTER * 0.5 + 0.01;
      let loss = if self.random.next_bound(3) == 0 { 2 } else { 1 };
      self.lightning(renderer, v3, rando, levels - loss);
      if self.random.next_bound(14) == 0 {
        let rando2 = angle - self.random.next_float() * ANGLE_JITTER - ANGLE_JITTER * 0.5;
        self.lightning(renderer, v3, rando2, levels - 1);
      }
    }
  }
}

impl Actor for Beato {
  fn get_position(&self) -> Vec2f { self.pos }
  fn set_position(&mut self, new_position: Vector2<f32>) { self.pos = new_position }

  fn update(&mut self, deltatime: f32) {
    const ACCELERATION: f32 = 3600.0;
    const FRICTION: f32 = 6.0;

    if let Some(pad) = GamePad::current() {
      let mut lstick = pad.left_stick().radial_deadzone(0.1, 1.0);

      if pad.down(PadButton::DPadLeft) { lstick.x -= 1.0 }
      if pad.down(PadButton::DPadRight) { lstick.x += 1.0 }
      if pad.down(PadButton::DPadUp) { lstick.y -= 1.0 }
      if pad.down(PadButton::DPadDown) { lstick.y += 1.0 }

      if lstick.mag() > 1.0 { lstick.normalise(); }
      self.vel += lstick * ACCELERATION * deltatime;
      if lstick.x < -0.1 {
        self.flip = true;
      } else if lstick.x > 0.1 {
        self.flip = false;
      }

      let rstick = pad.right_stick().radial_deadzone(0.1, 1.0);
      let lazer_mag = rstick.mag();
      if lazer_mag > 0.125 {
        self.lazering = if pad.right_trigger() >= 0.5 {
          self.lazervec = rstick / lazer_mag;
          true } else { false };
        self.flip = rstick.x < 0.0;
        if self.lazering {
          //GamePad.rumble = 1f32;
        }
      } else {
        self.lazering = false;
        //GamePad.rumble = 0f32;
      }
    }

    const RECT: Extent<i32> = Extent::new(-32, -48, 640 + 32, 480 + 48);

    self.pos += self.vel * deltatime;
    if self.pos.x < RECT.left as f32 {
      self.pos.x += RECT.width() as f32;
    }
    if self.pos.y < RECT.top as f32 {
      self.pos.y += RECT.height() as f32;
    }
    if self.pos.x >= RECT.right as f32 {
      self.pos.x -= RECT.width() as f32;
    }
    if self.pos.y >= RECT.bottom as f32 {
      self.pos.y -= RECT.height() as f32;
    }

    self.vel -= self.vel * FRICTION * deltatime;
  }

  fn draw(&mut self, renderer: &mut Renderer, _deltatime: f32) {
    let width: f32 = 48.0;
    let height: f32 = 64.0;
    let dst = FRect::new(
      self.pos.x - width * 0.5,
      self.pos.y - height * 0.5,
      width, height);
    let flip = if self.flip { Flip::Horizontal } else { Flip::None };
    let angle = self.vel.x as f64 / 120.0 * 2.45;
    renderer.copy(&self.guy_texture, dst, angle, flip);

    if self.lazering {
      self.lightning(renderer, self.pos, self.lazervec.angle(), 50);
    }
  }
}
