use std::io::Read;
use std::fs::File;

fn read_file(path: &str) -> String {
  let mut file = File::open(path).expect("File not found");
  let mut string = String::new();
  file.read_to_string(&mut string).expect("Read error");
  return string;
}

fn calc_sum(string: &str) -> u32 {
  let mut chars = string.chars().peekable();

  let mut sum = 0;
  let first = *chars.peek().expect("File empty");

  while let Some(c) = chars.next() {
    if c == *chars.peek().unwrap_or(&first) {
      sum += c.to_digit(10).expect("Not a digit");
    }
  }

  return sum;
}

fn main() {
  print!("{}\n", calc_sum(&read_file("input")));
}

#[test]
fn example_one() {
  assert_eq!(3, calc_sum("1122"));
}

#[test]
fn example_two() {
  assert_eq!(4, calc_sum("1111"));
}

#[test]
fn example_three() {
  assert_eq!(9, calc_sum("91212129"));
}
