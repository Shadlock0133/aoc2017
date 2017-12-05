pub fn day5_1(input: &str) -> u32 {
    let mut list: Vec<i32> = input.split_whitespace()
                    .filter_map(|word| word.parse::<i32>().ok())
                    .collect();
    let mut index = 0;
    let mut steps = 0;
    while let Some(cell) = list.get_mut(index) {
        steps += 1;
        index += *cell as usize;
        *cell += 1;
    }
    steps
}

pub fn day5_2(input: &str) -> u32 {
    let mut list: Vec<i32> = input.split_whitespace()
                    .filter_map(|word| word.parse::<i32>().ok())
                    .collect();
    let mut index = 0;
    let mut steps = 0;
    while let Some(cell) = list.get_mut(index) {
        steps += 1;
        index += *cell as usize;
        *cell += if *cell >= 3 {
            -1
        } else {
            1
        };
    }
    steps
}

#[cfg(test)]
mod part1 {
    use super::*;
    #[test] fn example1() { assert_eq!(day5_1("0 3 0 1 -3"), 5); }
}

#[cfg(test)]
mod part2 {
    use super::*;
    #[test] fn example1() { assert_eq!(day5_2("0 3 0 1 -3"), 10); }
}
