pub fn day1p1(input: &str) -> usize {
    let mut dial = 50;
    let mut cnt = 0;
    for line in input.lines() {
        let clicks: isize = line[1..].parse().unwrap();
        let signed_clicks = if line.starts_with('L') {
            -clicks
        } else {
            clicks
        };
        dial += signed_clicks;
        dial %= 100;

        if dial == 0 {
            cnt += 1;
        }
    }
    cnt
}

pub fn day1p2(input: &str) -> usize {
    let mut dial = 50;
    let mut cnt = 0;
    for line in input.lines() {
        let clicks: isize = line[1..].parse().unwrap();
        match line.chars().next().unwrap() {
            'R' => {
                if clicks >= 100 - dial {
                    cnt += ((clicks - (100 - dial)) / 100) + 1;
                }
                dial = (dial + clicks) % 100;
            }
            'L' => {
                if dial == 0 {
                    cnt += clicks / 100;
                } else if clicks >= dial {
                    cnt += ((clicks - dial) / 100) + 1;
                }
                dial = ((dial - clicks) % 100 + 100) % 100;
            }
            _ => panic!("unknown dial instruction"),
        }
    }
    cnt as usize
}

pub fn day2p1(input: &str) -> usize {
    input
        .split(',')
        .map(|s| {
            let mut parts = s.trim().split('-');
            let low = parts.next().unwrap().parse::<usize>().unwrap();
            let high = parts.next().unwrap().parse::<usize>().unwrap();
            (low, high)
        })
        .map(|(low, high)| {
            // this range only contains number with an odd number of digits, skip.
            if low.ilog10() == high.ilog10() && (low.ilog10() + 1) % 2 == 1 {
                0
            } else {
                (low..=high)
                    .filter(|i| {
                        let num_digits = if *i == 0 { 1 } else { i.ilog10() as usize + 1 };
                        if num_digits % 2 == 1 {
                            return false;
                        }
                        let half = (num_digits / 2) as u32;
                        let d = 10_usize.pow(half);
                        let left_half = i / d;
                        let right_half = i % d;
                        left_half == right_half
                    })
                    .sum::<usize>()
            }
        })
        .sum()
}

#[inline]
fn has_repeating_pattern(digits: &[u8], len: usize, pattern_len: usize) -> bool {
    for i in pattern_len..len {
        if digits[i] != digits[i % pattern_len] {
            return false;
        }
    }
    true
}
#[inline]
fn has_any_repeating_pattern(n: usize) -> bool {
    if n < 10 {
        return true;
    }
    // get all the digits and the length
    let mut digits = [0u8; 20]; // preallocate
    let mut len = 0;
    let mut temp = n;
    while temp > 0 {
        digits[len] = (temp % 10) as u8;
        temp /= 10;
        len += 1;
    }
    digits[..len].reverse();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }
        if has_repeating_pattern(&digits, len, pattern_len) {
            return true;
        }
    }
    false
}

pub fn day2p2(input: &str) -> usize {
    input
        .split(',')
        .map(|s| {
            let mut parts = s.trim().split('-');
            let low = parts.next().unwrap().parse::<usize>().unwrap();
            let high = parts.next().unwrap().parse::<usize>().unwrap();
            (low, high)
        })
        .map(|(low, high)| {
            (low..=high)
                .filter(|i| has_any_repeating_pattern(*i))
                .sum::<usize>()
        })
        .sum()
}

pub fn day3p1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let max = *bytes.iter().max().unwrap();
        let idx_max = bytes.iter().position(|&b| b == max).unwrap();
        let second_digit = if idx_max == bytes.len() - 1 {
            // max is at the end, find max in prefix
            bytes[..bytes.len() - 1].iter().max().unwrap().to_owned()
        } else {
            // find max after idx_max
            bytes[idx_max + 1..].iter().copied().max().unwrap()
        };
        sum += ((max - b'0') as usize * 10) + (second_digit - b'0') as usize;
    }
    sum
}

pub fn day3p2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut digits = Vec::with_capacity(12);
        let mut start = 0;
        for i in (0..12).rev() {
            let end = len - i;
            let max = bytes[start..end].iter().max().unwrap();
            let idx_max = bytes[start..end].iter().position(|c| c == max).unwrap();

            digits.push(*max);
            start = start + idx_max + 1;
        }
        unsafe {
            sum += String::from_utf8_unchecked(digits)
                .parse::<usize>()
                .unwrap();
        }
    }
    sum
}

// pub fn day4p1(input: &str) -> usize {
//     0
// }
//
// pub fn day4p2(input: &str) -> usize {
//     0
// }
//
// pub fn day5p1(input: &str) -> usize {
//     0
// }
//
// pub fn day5p2(input: &str) -> usize {
//     0
// }
//
// pub fn day6p1(input: &str) -> usize {
//     0
// }
//
// pub fn day6p2(input: &str) -> usize {
//     0
// }
//
// pub fn day7p1(input: &str) -> usize {
//     0
// }
//
// pub fn day7p2(input: &str) -> usize {
//     0
// }
//
// pub fn day8p1(input: &str) -> usize {
//     0
// }
//
// pub fn day8p2(input: &str) -> usize {
//     0
// }
//
// pub fn day9p1(input: &str) -> usize {
//     0
// }
//
// pub fn day9p2(input: &str) -> usize {
//     0
// }
//
// pub fn day10p1(input: &str) -> usize {
//     0
// }
//
// pub fn day10p2(input: &str) -> usize {
//     0
// }
//
// pub fn day11p1(input: &str) -> usize {
//     0
// }
//
// pub fn day11p2(input: &str) -> usize {
//     0
// }
//
// pub fn day12p1(input: &str) -> usize {
//     0
// }
//
// pub fn day12p2(input: &str) -> usize {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tday2p1() {
        assert_eq!(day2p1("11-22"), 33);
        assert_eq!(day2p1("95-115"), 99);
        assert_eq!(day2p1("1188511880-1188511890"), 1188511885);
    }

    #[test]
    fn tday2p2() {
        assert_eq!(day2p2("11-22"), 11 + 22);
        assert_eq!(day2p2("95-115"), 99 + 111);
        assert_eq!(day2p2("998-1012"), 999 + 1010);
        assert_eq!(day2p2("1188511880-1188511890"), 1188511885);
        assert_eq!(day2p2("222220-222224"), 222222);
        assert_eq!(day2p2("1698522-1698528"), 0);
        assert_eq!(day2p2("446443-446449"), 446446);
        assert_eq!(day2p2("38593856-38593862"), 38593859);
        assert_eq!(day2p2("565653-565659"), 565656);
        assert_eq!(day2p2("824824821-824824827"), 824824824);
        assert_eq!(day2p2("2121212118-2121212124"), 2121212121);
        assert_eq!(day2p2("11-22,95-115"), 33 + 210);
    }

    #[test]
    fn tday3p2() {
        assert_eq!(day3p2("987654321111111"), 987654321111);
        assert_eq!(day3p2("811111111111119"), 811111111119);
        assert_eq!(day3p2("234234234234278"), 434234234278);
        assert_eq!(day3p2("818181911112111"), 888911112111);
    }
}
