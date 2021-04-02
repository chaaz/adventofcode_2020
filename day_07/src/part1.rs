use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn run() {
  let content = include_str!("input.txt").trim();

  let rules: Vec<Rule> = content.split("\n").map(|l| l.parse().unwrap()).collect();
  let mut revmap = HashMap::new();
  for rule in rules {
    for (_, inner) in rule.inners {
      revmap.entry(inner.clone()).or_insert(HashSet::new()).insert(rule.outer.clone());
    }
  }

  let mut found = HashSet::new();
  let mut queue = Vec::new();
  let empty = HashSet::new();

  queue.push("shiny gold");
  while let Some(next) = queue.pop() {
    for targ in revmap.get(next).unwrap_or(&empty) {
      if !found.contains(targ) {
        found.insert(targ);
        queue.insert(0, targ);
      }
    }
  }

  println!("Answer: {}", found.len());
}

struct Rule {
  outer: String,
  inners: Vec<(u16, String)>
}

impl FromStr for Rule {
  type Err = ();
  fn from_str(v: &str) -> Result<Self, ()> {
    let mut lr = v.split("contain");
    let outer = lr.next().unwrap().trim().strip_suffix(" bags").unwrap().to_string();
    let r = lr.next().unwrap().trim();
    let inners = if r.contains("no other bags") {
      Vec::new()
    } else {
      r.split(",").map(|p| {
        let mut num_name = p.trim().splitn(2, ' ');
        let num = num_name.next().unwrap().parse().unwrap();
        let bags = num_name.next().unwrap().trim();
        let bags = bags.replace(".", "");
        let inner = bags.strip_suffix(" bags").or_else(|| bags.strip_suffix(" bag")).unwrap();
        (num, inner.to_string())
      }).collect()
    };

    Ok(Rule { outer, inners })
  }
}
