use utils::*;

pub fn day6_1(input: &str) -> u32 {
    let mut banks: Vec<u32> = input.split_whitespace().filter_map(parse).collect();
    let mut prevs = vec![banks.clone()];

    let mut steps = 0;
    loop {
        steps += 1;
        let max = banks.iter().max().unwrap().to_owned();
        let max_pos = banks.iter().position(|&x| x == max).unwrap().to_owned();

        let mut index = max_pos as usize;
        let blocks = ::std::mem::replace(banks.get_mut(index).unwrap(), 0);
        for _ in 0..blocks {
            index = (index + 1) % banks.len();
            banks[index] += 1;
        }

        if prevs.iter().find(|&x| x == &banks).is_none() {
            prevs.push(banks.clone());
        } else {
            break;
        }
    }
    steps
}

pub fn day6_2(input: &str) -> u32 {
    let mut banks: Vec<u32> = input.split_whitespace().filter_map(parse).collect();
    let mut prevs = vec![banks.clone()];

    loop {
        let max = banks.iter().max().unwrap().to_owned();
        let max_pos = banks.iter().position(|&x| x == max).unwrap().to_owned();

        let mut index = max_pos as usize;
        let blocks = ::std::mem::replace(banks.get_mut(index).unwrap(), 0);
        for _ in 0..blocks {
            index = (index + 1) % banks.len();
            banks[index] += 1;
        }

        match prevs.iter().position(|x| x == &banks) {
            Some(x) => {
                break (prevs.len() - x) as u32;
            }
            None => prevs.push(banks.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(day6_1("0 2 7 0"), 5);
    }

    #[test]
    fn part2() {
        assert_eq!(day6_2("0 2 7 0"), 4);
    }
}
