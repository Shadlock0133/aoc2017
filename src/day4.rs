pub fn day4_1(input: &str) -> u32 {
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let mut v: Vec<&str> = vec![];
            !line.split_whitespace()
                .any(|word| {
                    if v.iter().any(|&i| word == i) {
                        true
                    } else {
                        v.push(word);
                        false
                    }
                })
        })
        .count() as u32
}

fn are_anagrams(w1: &str, w2: &str) -> bool {
    let mut w1 = w1.chars().collect::<Vec<_>>();
    w1.sort_unstable();
    let mut w2 = w2.chars().collect::<Vec<_>>();
    w2.sort_unstable();
    w1 == w2
}

pub fn day4_2(input: &str) -> u32 {
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let mut v: Vec<&str> = vec![];
            !line.split_whitespace()
                .any(|word| {
                    if v.iter().any(|&i| are_anagrams(word, i)) {
                        true
                    } else {
                        v.push(word);
                        false
                    }
                })
        })
        .count() as u32
}

#[cfg(test)]
mod part1 {
    use super::*;
    #[test] fn example1() { assert_eq!(day4_1("aa bb cc dd ee"), 1); }
    #[test] fn example2() { assert_eq!(day4_1("aa bb cc dd aa"), 0); }
    #[test] fn example3() { assert_eq!(day4_1("aa bb cc dd aaa"), 1); }
    #[test] fn example4() {
        let input = 
            "aa bb cc dd ee
            aa bb cc dd aa
            aa bb cc dd aaa
            ";
        assert_eq!(day4_1(input), 2);
    }
}

#[cfg(test)]
mod part2 {
    use super::*;
    #[test] fn example1() { assert_eq!(day4_2("abcde fghij"), 1); }
    #[test] fn example2() { assert_eq!(day4_2("abcde xyz ecdab"), 0); }
    #[test] fn example3() { assert_eq!(day4_2("a ab abc abd abf abj"), 1); }
    #[test] fn example4() { assert_eq!(day4_2("iiii oiii ooii oooi oooo"), 1); }
    #[test] fn example5() { assert_eq!(day4_2("oiii ioii iioi iiio"), 0); }
}
