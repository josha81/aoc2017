use std::str::Lines;
use std::io::Read;
use std::fs::File;

fn read_file(path: &str) -> String {
  let mut file = File::open(path).expect("File not found");
  let mut string = String::new();
  file.read_to_string(&mut string).expect("Read error");
  return string;
}

fn calc_checksum(lines: Lines) -> u32 {
  let mut sum = 0;
  for line in lines {
    let mut ultimates = (u32::max_value(), 0);
    for field in line.split_whitespace() {
      let val = field.parse::<u32>().unwrap();
      if val < ultimates.0 {
        ultimates.0 = val;
      }
      if val > ultimates.1 {
        ultimates.1 = val;
      }
    }
    sum += ultimates.1 - ultimates.0;
  }
  return sum;
}

fn main() {
  print!("{}\n", calc_checksum(read_file("input").lines()));
}

#[test]
fn example() {
  let input = "\
    5 1 9 5
    7 5 3
    2 4 6 8";
  assert_eq!(18, calc_checksum(input.lines()));
}
