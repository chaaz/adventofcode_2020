pub fn run() {
  let content = include_str!("input.txt").trim();

  'out: for f1 in content.split("\n") {
    let f1: u32 = f1.parse().unwrap();
    for f2 in content.split("\n") {
      let f2: u32 = f2.parse().unwrap();
      if f1 + f2 == 2020 {
        println!("Answer: {}", f1 * f2);
        break 'out;
      }
    }
  }
}
