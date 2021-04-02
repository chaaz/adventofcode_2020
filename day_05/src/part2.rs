
pub fn run() {
  let content = include_str!("input.txt").trim();

  let mut all: Vec<_> = content
    .split('\n')
    .map(|pass| {
      let row = calc(&pass[.. 7]);
      let col = calc(&pass[7 ..]);
      row * 8 + col
    })
    .collect();
  all.sort();

  for w in all.windows(2) {
    if w[1] > w[0] + 1 {
      println!("ANSWER: {}", w[0] + 1);
      break;
    }
  }
}

fn calc(ptn: &str) -> usize {
  ptn
    .chars()
    .rev()
    .enumerate()
    .map(|(i, c)| if c == 'F' || c == 'L' { 0 } else { 2usize.pow(i as u32) })
    .sum()
}
