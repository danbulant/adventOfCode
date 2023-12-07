use std::{io::{BufReader, BufRead}, fs::File, ops::Index};

fn part1(file: &str) -> i32 {
    let file = BufReader::new(File::open(file).unwrap());
    let solution = file.lines()
        .map(|l| l.unwrap())
        .map(|l| (l.match_indices(char::is_numeric).next().unwrap().0, l.match_indices(char::is_numeric).last().unwrap().0, l))
        .map(|(start,end,l)| (l[start..start+1].to_string() + &l[end..end+1]))
        .map(|l| l.parse::<i32>().unwrap())
        .fold(0, |acc, x| acc + x);
    solution
}

fn part2(file: &str) -> i32 {
    let file = BufReader::new(File::open(file).unwrap());
    let inumbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let find_number = |s: &str| {
        if let Some(num) = inumbers.iter().position(|n| s.starts_with(n)) { Some(num) }
        else if let Some(num) = numbers.iter().position(|n| s.starts_with(n)) { Some(num) }
        else { None }
    };
    let solution = file.lines()
        .map(|l| l.unwrap() + "    ")
        .map(|l| {
            (l
                .chars()
                .collect::<Vec<char>>()
                .windows(5)
                .map(|w| w.iter().collect::<String>())
                .map(|w| find_number(&w))
                .filter(|w| w.is_some())
                .map(|w| w.unwrap())
                .collect::<Vec<usize>>(), l)
        })
        .map(|(nums, _)| nums.first().unwrap() * 10 + nums.last().unwrap())
        .fold(0, |acc, x| acc + x) as i32;

    solution
}

fn main() {
    dbg!(part2("input2"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_input1() {
        assert_eq!(part1("input1"), 142)
    }

    #[test]
    fn part1_input2() {
        assert_eq!(part1("input2"), 54597)
    }

    #[test]
    fn part2_input1() {
        assert_eq!(part2("input1"), 142)
    }

    #[test]
    fn part2_input2() {
        assert_eq!(part2("input2"), 54504)
    }

    #[test]
    fn part2_input3() {
        assert_eq!(part2("input3"), 281)
    }
}