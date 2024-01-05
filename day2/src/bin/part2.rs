use std::fs;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let input = fs::read_to_string("input.txt")?;
    println!("Answer: {}", calculate(&input));
    println!("Time taken: {:2?}", start.elapsed());
    
    Ok(())
}

fn calculate(input: &str) -> u32 {
  let line = input.lines().map(|line| line.split(": ").last().unwrap()).collect::<Vec<&str>>();
  let line_value: Vec<Vec<&str>> = line.iter().map(|line| line.split("; ").collect()).collect();
  let mut counter: u32 = 0;

  for (index, value) in line_value.iter().enumerate() {
    let rgb: Vec<_> = value.iter().flat_map(|&x| x.split(", ")).collect();
    let red = rgb.clone().into_iter().filter_map(|x| {
      if x.contains("red") {
        Some(x.split_whitespace().next().and_then(|y| y.parse::<u32>().ok()).unwrap())
      }
      else {
        None
      }
    }).collect::<Vec<u32>>();

    let green = rgb.clone().into_iter().filter_map(|x| {
      if x.contains("green") {
        Some(x.split_whitespace().next().and_then(|y| y.parse::<u32>().ok()).unwrap())
      }
      else {
        None
      }
    }).collect::<Vec<u32>>();

    let blue = rgb.clone().into_iter().filter_map(|x| {
      if x.contains("blue") {
        Some(x.split_whitespace().next().and_then(|y| y.parse::<u32>().ok()).unwrap())
      }
      else {
        None
      }
    }).collect::<Vec<u32>>();

    counter += red.iter().max().unwrap() * green.iter().max().unwrap() * blue.iter().max().unwrap();
  }

  counter
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part2() {
    let input = fs::read_to_string("test.txt").unwrap();
    assert_eq!(calculate(&input), 2286);
  }
}