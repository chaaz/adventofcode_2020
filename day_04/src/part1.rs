use std::str::FromStr;
use std::collections::HashSet;

pub fn run() {
  let content = include_str!("input.txt").trim();

  let list: List = content.parse().unwrap();

  println!("Answer: {}", list.valid);
}

struct List {
  valid: usize
}

impl FromStr for List {
  type Err = ();
  fn from_str(v: &str) -> Result<Self, ()> {
    let valid = v
      .split("\n\n")
      .filter(|block| {
        block
          .split(|c| c == ' ' || c == '\n')
          .map(|kv| kv.split(':').next().unwrap())
          .fold(Checker::new(), |c, k| c.with(k))
          .is_valid()
      })
      .count();

    Ok(List { valid })
  }
}

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const HGT: &str = "hgt";
const HCL: &str = "hcl";
const ECL: &str = "ecl";
const PID: &str = "pid";

struct Checker {
  still_reqd: HashSet<&'static str>
}

impl Checker {
  pub fn new() -> Checker {
    let mut still_reqd = HashSet::new();
    still_reqd.insert(BYR);
    still_reqd.insert(IYR);
    still_reqd.insert(EYR);
    still_reqd.insert(HGT);
    still_reqd.insert(HCL);
    still_reqd.insert(ECL);
    still_reqd.insert(PID);

    Checker { still_reqd }
  }

  fn with(mut self, val: &str) -> Checker {
    self.still_reqd.retain(|v| v != &val);
    self
  }

  fn is_valid(&self) -> bool { self.still_reqd.is_empty() }
}
