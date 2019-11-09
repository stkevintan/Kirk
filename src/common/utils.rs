use std::borrow::Cow;

//#7057ff
pub fn is_light_color(color: &str) -> bool {
  let color = color.trim_start_matches("#");
  let x = i64::from_str_radix(&fill(color), 16).unwrap();
  let r = (x >> 16) as f64;
  let g = (x >> 8 & 255) as f64;
  let b = (x & 255) as f64;
  let hsp = (0.299 * r * r + 0.587 * g * g + 0.114 * b * b).sqrt();
  hsp > 127.5
}

fn fill<'a>(color: &'a str) -> Cow<'a, str> {
  if color.len() < 4 {
    let color = color.chars().flat_map(|c| vec![c; 2]).collect::<String>();
    return Cow::Owned(color);
  } else {
    return Cow::Borrowed(color);
  }
}
