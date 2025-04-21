pub(crate) struct Colour {
  pub(crate) r: u8,
  pub(crate) g: u8,
  pub(crate) b: u8,
  pub(crate) a: u8
}

#[allow(dead_code)]
impl Colour {
  pub(crate) const fn rgb(r: u8, g: u8, b: u8) -> Self { Self { r, g, b, a: 0xFF } }
  pub(crate) const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
  pub(crate) const fn hex(rgba: u32) -> Colour { Self {
    r: ((rgba & 0xFF000000u32) >> 24) as u8,
    g: ((rgba & 0x00FF0000u32) >> 16) as u8,
    b: ((rgba & 0x0000FF00u32) >>  8) as u8,
    a:  (rgba & 0x000000FFu32)        as u8,
  } }

  pub(crate) const BLACK: Self = Self::rgb(0x00, 0x00, 0x00);
  pub(crate) const WHITE: Self = Self::rgb(0xFF, 0xFF, 0xFF);
  pub(crate) const RED: Self = Self::rgb(0xFF, 0x00, 0x00);
}
