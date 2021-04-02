use std::collections::HashMap;
use std::str::FromStr;

pub fn run() {
  let content = include_str!("input.txt").trim();

  let rules: Vec<Rule> = content.split("\n").map(|l| l.parse().unwrap()).collect();
  let mut formap = HashMap::new();
  for rule in rules {
    formap.insert(rule.outer.clone(), rule.inners.clone());
  }

  let mut found = HashMap::new();
  let total = find("shiny gold".to_string(), &mut found, &formap);

  println!("Answer: {}", total);
}

fn find<'a>(
  outer: String, found: &'a mut HashMap<String, usize>, rules: &'a HashMap<String, Vec<(usize, String)>>
) -> usize {
  if let Some(c) = found.get(&outer) {
    *c
  } else {
    let total = rules[&outer].iter().map(|(isz, iname)| {
      let total = isz * (1 + find(iname.to_string(), found, rules));
      total
    }).sum();
    found.insert(outer, total);
    total
  }
}

struct Rule {
  outer: String,
  inners: Vec<(usize, String)>
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
