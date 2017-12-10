use utils::*;

pub fn day2_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let numbers = line.split_whitespace().filter_map(parse);
            let min = numbers.clone().min();
            let max = numbers.max();
            match (min, max) {
                (Some(min), Some(max)) => Some(max - min),
                _ => None,
            }
        })
        .sum()
}

pub fn day2_2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let numbers = line.split_whitespace().filter_map(parse);
            for x in numbers.clone() {
                for y in numbers.clone() {
                    if x != y && x % y == 0 {
                        return Some(x / y);
                    }
                }
            }
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "5 1 9 5
            7 5 3
            2 4 6 8";
        assert_eq!(day2_1(input), 18);
    }

    #[test]
    fn part2() {
        let input = "5 9 2 8
            9 4 7 3
            3 8 6 5";
        assert_eq!(day2_2(input), 9);
    }
}
