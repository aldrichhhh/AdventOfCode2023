use std::fs;
use std::time::Instant;

fn main() -> std::io::Result<()>{
    let start = Instant::now();
    let input = fs::read_to_string("input.txt")?;

    println!("Answer: {}", calculate(&input));
    println!("Time taken: {:2?}", start.elapsed());
    Ok(())
}

fn calculate(input: &str) -> u32 {
    let mut result: u32 = 0;
    let numbers = ['0','1','2','3','4','5','6','7','8','9'];
    let mut lines: Vec<&str> = input.lines().collect();
    
    for line in &mut lines {
        for _ in line.chars() {
            if !line.starts_with(numbers) {
                *line = &line[1..]
            }
        }

        for _ in line.chars() {
            if !line.ends_with(numbers) {
                *line = &line[..line.len() - 1]
            }
        }
        let formatted_value = format!("{}{}", line.chars().next().unwrap(), line.chars().last().unwrap()).parse::<u32>().unwrap();
        result += formatted_value;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_part1.txt").unwrap();
        assert_eq!(calculate(&input), 142);
    }
}