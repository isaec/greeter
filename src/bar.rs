use termion::color;

pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
  }
  fn as_term_rgb(&self) -> color::Rgb {
    color::Rgb(self.r, self.g, self.b)
  }
  fn blend_channel(a: u8, b: u8, ratio: f32) -> u8 {
    (((1.0 - ratio) * (a as u16).pow(2) as f32) + (ratio * (b as u16).pow(2) as f32)).sqrt() as u8
  }
  fn blend(a: &Color, b: &Color, ratio: f32) -> Color {
    println!("test: {}", Color::blend_channel(60, 100, 0.5));
    Color {
      r: Color::blend_channel(a.r, b.r, ratio),
      g: Color::blend_channel(a.g, b.g, ratio),
      b: Color::blend_channel(a.b, b.b, ratio),
    }
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
    color::Fg(Color::blend(&self.empty, &self.full, value).as_term_rgb())
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
