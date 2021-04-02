use std::str::FromStr;

pub fn run() {
  let content = include_str!("input.txt").trim();

  let grid: Grid = content.parse().unwrap();

  println!("Answer: {}", grid.count_trees(3, 1));
}

struct Grid {
  rows: Vec<Row>
}

impl Grid {
  fn at(&self, row: usize, col: usize) -> char { self.rows.iter().nth(row).unwrap().at(col)  }
  fn tree(&self, row: usize, col: usize) -> bool { self.at(row, col) == '#' }

  fn count_trees(&self, right: usize, down: usize) -> usize {
    let mut trees = 0;
    let (mut r, mut c): (usize, usize) = (0, 0);
    while r < self.rows.len() {
      if self.tree(r, c) {
        trees += 1;
      }

      r += down;
      c += right;
      c = c % self.rows[0].data.len();
    }
    trees
  }
}

impl FromStr for Grid {
  type Err = ();
  fn from_str(v: &str) -> Result<Self, ()> {
    Ok(Grid { rows: v.split('\n').map(|l| l.parse().unwrap()).collect() })
  }
}

struct Row {
  data: String
}

impl Row {
  fn at(&self, col: usize) -> char { self.data.chars().nth(col).unwrap() }
}

impl FromStr for Row {
  type Err = ();
  fn from_str(v: &str) -> Result<Self, ()> { Ok(Row { data: v.trim().to_string() }) }
}
