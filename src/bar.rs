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
  fn blend_two_channel(a: u8, b: u8, ratio: f32) -> u8 {
    (((1.0 - ratio) * (a as u16).pow(2) as f32) + (ratio * (b as u16).pow(2) as f32)).sqrt() as u8
  }
  fn blend_three_channel(a: u8, b: u8, c: u8, ratio: f32) -> u8 {
    Color::blend_two_channel(
      Color::blend_two_channel(a, b, ratio),
      Color::blend_two_channel(b, c, ratio),
      ratio,
    )
  }
  fn blend_two(a: &Color, b: &Color, ratio: f32) -> Color {
    Color {
      r: Color::blend_two_channel(a.r, b.r, ratio),
      g: Color::blend_two_channel(a.g, b.g, ratio),
      b: Color::blend_two_channel(a.b, b.b, ratio),
    }
  }
  fn blend_three(a: &Color, b: &Color, c: &Color, ratio: f32) -> Color {
    Color {
      r: Color::blend_three_channel(a.r, b.r, c.r, ratio),
      g: Color::blend_three_channel(a.g, b.g, c.g, ratio),
      b: Color::blend_three_channel(a.b, b.b, c.b, ratio),
    }
  }
}

pub struct ColorRange {
  empty: Color,
  mid: Option<Color>,
  full: Color,
}

impl ColorRange {
  pub fn new2(empty: Color, full: Color) -> ColorRange {
    ColorRange {
      empty,
      mid: None,
      full,
    }
  }
  pub fn new3(empty: Color, mid: Color, full: Color) -> ColorRange {
    ColorRange { empty, mid: Some(mid), full }
  }
  pub fn get_color(&self, value: f32) -> color::Fg<color::Rgb> {
    match &self.mid {
      Some(mid) => color::Fg(Color::blend_three(&self.empty, mid, &self.full, value).as_term_rgb()),
      None => color::Fg(Color::blend_two(&self.empty, &self.full, value).as_term_rgb())
    }
  }
}

pub fn make(
  length: u8,
  value: u16, // 0 to 100
  color_range: &ColorRange,
  left_cap: &str,
  full_char: &str,
  empty_char: &str,
  right_cap: &str,
  reset: &color::Fg<color::Rgb>,
) -> String {
  let inner_chars = (length - 2) as f32;
  let value_dec = value as f32 / 100.0;

  format!(
    "{left_cap}{color}{full_chars}{reset}{empty_chars}{right_cap}",
    full_chars = full_char.repeat((inner_chars * value_dec).ceil() as usize),
    empty_chars = empty_char.repeat((inner_chars * (1.0 - value_dec)).floor() as usize),
    left_cap = left_cap,
    color = color_range.get_color(value_dec),
    reset = reset,
    right_cap = right_cap
  )
}
