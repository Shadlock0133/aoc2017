pub fn day1_1(input: &str) -> u32 {
    let mut list: Vec<u8> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as u8)
        .collect();
    if let Some(&x) = list.get(0) {
        list.push(x);
    }
    list.windows(2)
        .filter(|&x| x[0] == x[1])
        .fold(0, |a, x| {
            a + x[0] as u32
        })
}

pub fn day1_2(input: &str) -> u32 {
    let list: Vec<u8> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as u8)
        .collect();
    let (left, right) = list.split_at(list.len() / 2);
    left.iter().zip(right.iter())
        .filter(|&(x, y)| x == y)
        .fold(0, |a, (&x, &y)| {
            a + x as u32 + y as u32
        })
}

#[cfg(test)]
mod part1 {
    use super::*;
    #[test] fn example1() { assert_eq!(day1_1("1122"), 3); }
    #[test] fn example2() { assert_eq!(day1_1("1111"), 4); }
    #[test] fn example3() { assert_eq!(day1_1("1234"), 0); }
    #[test] fn example4() { assert_eq!(day1_1("91212129"), 9); }
}

#[cfg(test)]
mod part2 {
    use super::*;
    #[test] fn example1() { assert_eq!(day1_2("1212"), 6); }
    #[test] fn example2() { assert_eq!(day1_2("1221"), 0); }
    #[test] fn example3() { assert_eq!(day1_2("123425"), 4); }
    #[test] fn example4() { assert_eq!(day1_2("123123"), 12); }
    #[test] fn example5() { assert_eq!(day1_2("12131415"), 4); }
}