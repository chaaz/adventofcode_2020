use std::collections::HashSet;

pub fn run() {
  let content = include_str!("input.txt").trim();

  let sum: usize = content.split("\n\n").map(process_group).sum();
  println!("Answer: {}", sum);
}

fn process_group(group: &str) -> usize {
  let set = group.split('\n').enumerate().fold(HashSet::new(), |mut set, (i, pers)| {
    if i == 0 {
      for c in pers.chars() {
        set.insert(c);
      }
    } else {
      let cmp: HashSet<char> = pers.chars().collect();
      set.retain(|c| cmp.contains(c));
    }
    set
  });

  set.len()
}
