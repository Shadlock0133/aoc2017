use utils::*;

pub fn day3_1(input: u32) -> u32 {
    let wheel = ((((input - 1) as f32).sqrt().floor() as u32) + 1) / 2;
    let start = ((wheel * 2).saturating_sub(1)).pow(2);
    let end = ((wheel * 2).saturating_add(1)).pow(2);
    let pos = input - start;
    let side_len = (end - start) / 4;
    wheel + zigzag(pos + (side_len / 2), side_len / 2)
}

pub fn day3_2(input: u32) -> u32 {
    if input == 1 { return 1; }
    let wheel = ((((input - 1) as f32).sqrt().floor() as u32) + 1) / 2;
    let start = ((wheel * 2).saturating_sub(1)).pow(2);
    let end = ((wheel * 2).saturating_add(1)).pow(2);
    let _pos = input - start;
    let _side_len = (end - start) / 4;
    return day3_2(input - 1);
}

#[cfg(test)]
mod part1 {
    use super::*;
    #[test] fn example1() { assert_eq!(day3_1(1), 0); }
    #[test] fn example2() { assert_eq!(day3_1(12), 3); }
    #[test] fn example3() { assert_eq!(day3_1(23), 2); }
    #[test] fn example4() { assert_eq!(day3_1(1024), 31); }
}

#[cfg(test)]
mod part2 {
    use super::*;
    #[ignore] #[test] fn example1() { assert_eq!(day3_2(1), 1); }
    #[ignore] #[test] fn example2() { assert_eq!(day3_2(2), 1); }
    #[ignore] #[test] fn example3() { assert_eq!(day3_2(3), 2); }
    #[ignore] #[test] fn example4() { assert_eq!(day3_2(4), 4); }
    #[ignore] #[test] fn example5() { assert_eq!(day3_2(5), 5); }
    #[ignore] #[test] fn example6() { assert_eq!(day3_2(23), 806); }
}
