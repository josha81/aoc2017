use std::io::Read;
use std::fs::File;

fn read_file(path: &str) -> String {
  let mut file = File::open(path).expect("File not found");
  let mut string = String::new();
  file.read_to_string(&mut string).expect("Read error");
  return string;
}

fn calc_sum(string: &str) -> u32 {
  let mut chars = string.chars();
  let mut halfway = chars.clone().cycle().skip(string.len() / 2);
  let mut sum = 0;

  while let Some(c) = chars.next() {
    if c == halfway.next().unwrap() {
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
  assert_eq!(6, calc_sum("1212"));
}

#[test]
fn example_two() {
  assert_eq!(0, calc_sum("1221"));
}

#[test]
fn example_three() {
  assert_eq!(4, calc_sum("123425"));
}

#[test]
fn example_four() {
  assert_eq!(12, calc_sum("123123"));
}

#[test]
fn example_five() {
  assert_eq!(4, calc_sum("12131415"));
}
