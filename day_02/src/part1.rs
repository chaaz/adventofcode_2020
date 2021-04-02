use std::str::FromStr;

pub fn run() {
  let content = include_str!("input.txt").trim();

  let valid_count = content
    .split("\n")
    .filter(|line| {
      let mut parts = line.split(':');
      let policy: Policy = parts.next().unwrap().parse().unwrap();
      let pw = parts.next().unwrap().trim();
      policy.is_valid(pw)
    })
    .count();

  println!("Answer: {}", valid_count);
}

struct Policy {
  letter: char,
  min: usize,
  max: usize
}

impl Policy {
  pub fn is_valid(&self, pw: &str) -> bool {
    let count = pw.chars().filter(|l| l == &self.letter).count();
    count >= self.min && count <= self.max
  }
}

impl FromStr for Policy {
  type Err = ();
  fn from_str(v: &str) -> Result<Self, ()> {
    let mut parts = v.split(' ');
    let range = parts.next().unwrap();
    let letter = parts.next().unwrap();

    let mut minmax = range.split('-');
    let min = minmax.next().unwrap().parse().unwrap();
    let max = minmax.next().unwrap().parse().unwrap();

    Ok(Policy { min, max, letter: letter.chars().nth(0).unwrap() })
  }
}
