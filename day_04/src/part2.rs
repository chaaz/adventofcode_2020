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
          .map(|kv| {
            let mut kv = kv.split(':');
            let k = kv.next().unwrap();
            let v = kv.next().unwrap();
            (k, v)
          })
          .fold(Checker::new(), |c, (k, v)| c.with(k, v))
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

  fn with(mut self, key: &str, val: &str) -> Checker {
    if match key {
      "byr" => if let Ok(val) = val.parse::<u16>() {
          val >= 1920 && val <= 2002
        } else { false },
      "iyr" => if let Ok(val) = val.parse::<u16>() {
          val >= 2010 && val <= 2020
        } else { false },
      "eyr" => if let Ok(val) = val.parse::<u16>() {
          val >= 2020 && val <= 2030
        } else { false },
      "hgt" => if let Some(rem) = val.strip_suffix("cm") {
          is_height(rem, 150, 193)
        } else if let Some(rem) = val.strip_suffix("in") {
          is_height(rem, 59, 76)
        } else { false },
      "hcl" => if let Some(rem) = val.strip_prefix("#") {
          is_hcl(rem)
        } else { false },
      "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|c| c == &val),
      "pid" => val.len() == 9 && val.chars().all(|c| c.is_digit(10)),
      _ => false
    } {
      self.still_reqd.retain(|k| k != &key);
    }
    self
  }

  fn is_valid(&self) -> bool { self.still_reqd.is_empty() }
}

fn is_height(v: &str, min: u16, max: u16) -> bool {
  if let Ok(v) = v.parse::<u16>() {
    v >= min && v <= max
  } else { false }
}

fn is_hcl(v: &str) -> bool {
  v.chars().all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
}
