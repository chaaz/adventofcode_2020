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
  first: usize,
  second: usize
}

impl Policy {
  pub fn is_valid(&self, pw: &str) -> bool {
    let at1 = pw.chars().nth(self.first - 1).unwrap();
    let at2 = pw.chars().nth(self.second - 1).unwrap();
    (at1 == self.letter && at2 != self.letter)
      || (at1 != self.letter && at2 == self.letter)
  }
}

impl FromStr for Policy {
  type Err = ();
  fn from_str(v: &str) -> Result<Self, ()> {
    let mut parts = v.split(' ');
    let range = parts.next().unwrap();
    let letter = parts.next().unwrap();

    let mut inds = range.split('-');
    let first = inds.next().unwrap().parse().unwrap();
    let second = inds.next().unwrap().parse().unwrap();

    Ok(Policy { first, second, letter: letter.chars().nth(0).unwrap() })
  }
}
