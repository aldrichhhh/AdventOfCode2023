use std::fs;
use std::collections::HashMap;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let input = fs::read_to_string("input.txt")?;

    println!("Answer: {}", calculate(&input));
    println!("Time taken: {:2?}", start.elapsed());
    Ok(())
}

fn calculate(input: &str) -> u32 {
    let num_digit = ['0','1','2','3','4','5','6','7','8','9'];
    let mut num = HashMap::from([
    ("zero", "0"),
    ("one", "1"),   
    ("two", "2"),    
    ("three", "3"),    
    ("four", "4"),   
    ("five", "5"),    
    ("six", "6"),   
    ("seven", "7"),    
    ("eight", "8"),    
    ("nine", "9"),
    ]);


    let mut lines: Vec<&str> = input.lines().collect();
    let mut result:u32 = 0;


    for line in &mut lines {
        if !line.starts_with(num_digit) {
            for _ in line.chars() {
                if !num.keys().any(|number| line.starts_with(number) || line.starts_with(num_digit)) {
                    *line = &line[1..]
                }
            }
        }
        if !line.ends_with(num_digit) {
            for _ in line.chars() {
                if !num.keys().any(|number| line.ends_with(number) || line.ends_with(num_digit)) {
                    *line = &line[..line.len() - 1];
                }
            }
        }

        let mut line = line.to_string();

        for key in num.keys() {
            if line.starts_with(key) {
                if let Some(pos) = line.find(key) {
                    line.replace_range(pos..pos + key.len(), num.get(key).unwrap());
                }
            }
            if line.ends_with(key) {
                if let Some(pos) = line.rfind(key) {
                    line.replace_range(pos..pos + key.len(), num.get(key).unwrap());
                }
            }
        }
        
        let value = (line.chars().next().unwrap().to_string() + line.pop().unwrap().to_string().as_str()).parse::<u32>().unwrap();
        result += value;
     }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_part2.txt").unwrap();
        assert_eq!(calculate(&input), 281)
    }
}