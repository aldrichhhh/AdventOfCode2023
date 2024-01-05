use std::fs;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let input = fs::read_to_string("input.txt")?;
    println!("Answer: {}", calculate(&input));
    println!("Time taken: {:2?}", start.elapsed());

    Ok(())
}

fn calculate(input: &str) -> usize {
  let max_red = 12;
  let max_green = 13;
  let max_blue = 14;
  let mut failure_counter: usize = 0;
  let file_size: usize = input.lines().count();
  let counter: usize = (1..=file_size).sum();

  let line = input.lines().map(|line| line.split(": ").last().unwrap()).collect::<Vec<&str>>();
  let line_value: Vec<Vec<&str>> = line.iter().map(|line| line.split("; ").collect()).collect();

  for (index, value) in line_value.iter().enumerate() {
    let rgb: Vec<_> = value.iter().flat_map(|&x| x.split(", ")).collect();

    for colors in rgb {
      let quantity = colors.split_whitespace().next().and_then(|x| x.parse::<u32>().ok()).unwrap();
      let color = colors.split_whitespace().last().unwrap();
      if color == "red" && quantity > max_red || color == "green" && quantity > max_green || color == "blue" && quantity > max_blue {
        failure_counter += index+1;
        break
      }
    }
  }
  let result = counter - failure_counter;
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let input = fs::read_to_string("test.txt").unwrap();
    assert_eq!(calculate(&input), 8)      
  }
}