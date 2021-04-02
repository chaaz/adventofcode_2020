
pub fn run() {
  let content = include_str!("input.txt").trim();

  let max = content
    .split('\n')
    .map(|pass| {
      let row = calc(&pass[.. 7]);
      let col = calc(&pass[7 ..]);
      row * 8 + col
    })
    .max();

  println!("Answer: {}", max.unwrap());
}

fn calc(ptn: &str) -> usize {
  ptn
    .chars()
    .rev()
    .enumerate()
    .map(|(i, c)| if c == 'F' || c == 'L' { 0 } else { 2usize.pow(i as u32) })
    .sum()
}
