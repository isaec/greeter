pub fn make(
  length: u8,
  value: u16, // 0 to 100
  left_cap: &str,
  full_char: &str,
  empty_char: &str,
  right_cap: &str,
) -> String {
  let inner_chars = (length - 2) as f32;
  let value_dec = value as f32 / 100.0;

  format!(
    "{left_cap}{full_chars}{empty_chars}{right_cap}",
    full_chars = full_char.repeat((inner_chars * value_dec).ceil() as usize),
    empty_chars = empty_char.repeat((inner_chars * (1.0 - value_dec)).floor() as usize),
    left_cap = left_cap,
    right_cap = right_cap
  )
}
