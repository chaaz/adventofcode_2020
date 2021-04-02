use std::collections::HashSet;

pub fn run() {
  let content = include_str!("input.txt").trim();

  let sum: usize = content.split("\n\n").map(process_group).sum();
  println!("Answer: {}", sum);
}

fn process_group(group: &str) -> usize {
  let set = group.split('\n').fold(HashSet::new(), |mut set, pers| {
    for c in pers.chars() {
      set.insert(c);
    }
    set
  });

  set.len()
}
