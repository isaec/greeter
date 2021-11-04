use termion::color;

pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  pub fn rgb(r: u8, b: u8, g: u8) -> Color {
    Color { r, g, b }
  }
}

pub struct ColorRange {
  empty: Color,
  full: Color,
}

impl ColorRange {
  pub fn new(empty: Color, full: Color) -> ColorRange {
    ColorRange { empty, full }
  }
  fn get_color(&self, value: f32) -> color::Fg<color::Rgb> {
    color::Fg(color::Rgb(
      ((self.full.r as f32 * value) + (self.empty.r as f32 * 1.0 - value)) as u8,
      ((self.full.g as f32 * value) + (self.empty.g as f32 * 1.0 - value)) as u8,
      ((self.full.b as f32 * value) + (self.empty.b as f32 * 1.0 - value)) as u8,
    ))
  }
}

pub fn make(
  length: u8,
  value: u16, // 0 to 100
  color_range: ColorRange,
  left_cap: &str,
  full_char: &str,
  empty_char: &str,
  right_cap: &str,
) -> String {
  let inner_chars = (length - 2) as f32;
  let value_dec = value as f32 / 100.0;

  format!(
    "{left_cap}{color}{full_chars}{reset}{empty_chars}{right_cap}",
    full_chars = full_char.repeat((inner_chars * value_dec).ceil() as usize),
    empty_chars = empty_char.repeat((inner_chars * (1.0 - value_dec)).floor() as usize),
    left_cap = left_cap,
    color = color_range.get_color(value_dec),
    reset = color::Fg(color::Reset),
    right_cap = right_cap
  )
}
