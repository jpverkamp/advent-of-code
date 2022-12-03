use std::path::Path;
use aoc::*;

fn read(filename: &Path) -> Vec<u32> {
    let mut calories = Vec::new();
    let mut current = 0;

    for line in read_lines(filename) {
        if line.len() == 0 {
            calories.push(current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }
    calories.push(current);

    return calories;
}

fn part1(filename: &Path) -> String {
    let calories = read(filename);
    calories.iter().max().expect("no calories found, can't take max").to_string()
}

fn part2(filename: &Path) -> String {
    let mut calories = read(filename);
    
    calories.sort();
    calories.reverse();

    calories.iter().take(3).sum::<u32>().to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_day01_part1() {
        let expected = String::from("70369");
        let actual = crate::part1(Path::new("data/01.txt"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day01_part2() {
        let expected = String::from("203002");
        let actual = crate::part2(Path::new("data/01.txt"));

        assert_eq!(expected, actual);
    }
}
