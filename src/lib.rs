use std::collections::HashSet;
use std::io::BufRead;

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

#[inline]
fn count_neighbors(grid: &[Vec<u8>], i: usize, j: usize, num_rows: usize, num_cols: usize) -> u8 {
    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    DIRS.iter()
        .filter_map(|&(di, dj)| {
            let ni = i.wrapping_add_signed(di);
            let nj = j.wrapping_add_signed(dj);
            (ni < num_rows && nj < num_cols).then(|| grid[ni][nj])
        })
        .sum()
}

pub fn day4p1(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| (b == b'@') as u8).collect())
        .collect();

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut sum = 0;

    for i in 0..num_rows {
        for j in 0..num_cols {
            if grid[i][j] == 1 && count_neighbors(&grid, i, j, num_rows, num_cols) < 4 {
                sum += 1;
            }
        }
    }
    sum
}

pub fn day4p2(input: &str) -> usize {
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| (b == b'@') as u8).collect())
        .collect();

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut sum = 0;
    let mut to_remove: Vec<(usize, usize)> = Vec::with_capacity(2048);

    loop {
        to_remove.clear();

        for i in 0..num_rows {
            for j in 0..num_cols {
                if grid[i][j] == 1 && count_neighbors(&grid, i, j, num_rows, num_cols) < 4 {
                    to_remove.push((i, j));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        sum += to_remove.len();
        for &(i, j) in &to_remove {
            grid[i][j] = 0;
        }
    }

    sum
}

pub fn day5p1(input: &str) -> usize {
    let (seg_intervals, seg_ids) = input.split_once("\n\n").unwrap();
    let mut intervals = seg_intervals
        .lines()
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            Some((start.parse().ok()?, end.parse().ok()?))
        })
        .collect::<Vec<(usize, usize)>>();
    intervals.sort_unstable();

    let mut merged = Vec::new();
    for (start, end) in &intervals {
        if let Some((ml, mr)) = merged.last_mut() {
            if start <= *mr {
                *mr = (*mr).max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    seg_ids
        .lines()
        .filter(|line| {
            let id: usize = line.parse().unwrap();
            merged
                .binary_search_by(|&(l, r)| {
                    if id < *l {
                        std::cmp::Ordering::Greater
                    } else if id > *r {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
        })
        .count()
}

pub fn day5p2(input: &str) -> usize {
    let (seg_intervals, seg_ids) = input.split_once("\n\n").unwrap();
    let mut intervals = seg_intervals
        .lines()
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            Some((start.parse().ok()?, end.parse().ok()?))
        })
        .collect::<Vec<(usize, usize)>>();
    intervals.sort_unstable();

    let mut merged = Vec::new();
    for (start, end) in &intervals {
        if let Some((ml, mr)) = merged.last_mut() {
            if start <= *mr {
                *mr = (*mr).max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    merged.iter().map(|&(l, r)| r - l + 1).sum()
}

pub fn day6p1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<Vec<usize>> = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.trim()
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect();

    let mut ops: Vec<char> = vec![];
    for c in lines.last().unwrap().chars() {
        if c == '*' {
            ops.push(c);
        } else if c == '+' {
            ops.push(c);
        }
    }

    let mut sum = 0;
    for i in 0..numbers[0].len() {
        let mut tmp;
        if ops[i] == '*' {
            tmp = 1;
            for j in 0..numbers.len() {
                tmp *= numbers[j][i];
            }
        } else if ops[i] == '+' {
            tmp = 0;
            for j in 0..numbers.len() {
                tmp += numbers[j][i];
            }
        } else {
            panic!("unknown op");
        }
        sum += tmp;
    }

    sum
}

fn get_col(rows: &[&[u8]], idx_col: usize) -> Option<usize> {
    let mut res = None;
    let mut multiplier = 1;

    for row in rows.iter().rev() {
        if idx_col < row.len() && row[idx_col].is_ascii_digit() {
            let digit = (row[idx_col] - b'0') as usize;

            if let Some(val) = res {
                res = Some(val + digit * multiplier);
            } else {
                res = Some(digit);
            }
            multiplier *= 10;
        }
    }
    res
}

pub fn day6p2(input: &str) -> usize {
    let rows: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let data_rows: Vec<&[u8]> = rows[..rows.len() - 1].to_vec(); // exclude operator row
    let ops = rows.last().unwrap();

    let mut sum = 0;
    let mut tmp = None;
    let mut current_op = None;

    for i in 0..ops.len() {
        let ch = ops[i];

        if ch == b'*' || ch == b'+' {
            if let (Some(op), Some(val)) = (current_op, tmp) {
                sum += val;
            }
            current_op = Some(ch);
            tmp = if ch == b'*' { Some(1) } else { Some(0) };
        }
        {
            if let Some(col_val) = get_col(&data_rows, i) {
                if let Some(op) = current_op {
                    if op == b'*' {
                        tmp = Some(tmp.unwrap_or(1) * col_val);
                    } else if op == b'+' {
                        tmp = Some(tmp.unwrap_or(0) + col_val);
                    }
                }
            }
        }
    }

    if let Some(val) = tmp {
        sum += val;
    }

    sum
}

pub fn day7p1(input: &str) -> usize {
    let rows: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    // idea: track the current rows beams counting the number of splits
    let start = rows[0].iter().position(|&c| c == b'S').unwrap();
    let mut beams = HashSet::new();
    beams.insert((1, start));

    let mut sum = 0;

    for i in 1..(rows.len() - 1) {
        let mut new_beams = HashSet::new();
        for beam in &beams {
            // is there a splitter in the path of this beam?
            if rows[beam.0 + 1][beam.1] == b'^' {
                sum += 1;
                new_beams.insert((beam.0 + 1, beam.1 - 1)); // left 
                new_beams.insert((beam.0 + 1, beam.1 + 1)); // right
            } else {
                new_beams.insert((beam.0 + 1, beam.1));
            }
        }
        beams = new_beams;
    }
    sum
}

pub fn day7p2(input: &str) -> usize {
    let rows: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let start = rows[0].iter().position(|&c| c == b'S').unwrap();
    let mut beams = HashSet::new();
    beams.insert((1, start));

    let mut path_counts = vec![vec![0; rows[0].len()]; rows.len()];
    path_counts[1][start] = 1;

    // idea: each grid position tracks how many distinct paths make it to it
    for i in 1..(rows.len() - 1) {
        let mut new_beams = HashSet::new();
        for beam in &beams {
            // is there a splitter in the path of this beam?
            if rows[beam.0 + 1][beam.1] == b'^' {
                new_beams.insert((beam.0 + 1, beam.1 - 1)); // left 
                new_beams.insert((beam.0 + 1, beam.1 + 1)); // right
                path_counts[beam.0 + 1][beam.1 - 1] += path_counts[beam.0][beam.1];
                path_counts[beam.0 + 1][beam.1 + 1] += path_counts[beam.0][beam.1];
            } else {
                new_beams.insert((beam.0 + 1, beam.1));
                path_counts[beam.0 + 1][beam.1] += path_counts[beam.0][beam.1];
            }
        }
        beams = new_beams;
    }
    path_counts.last().unwrap().iter().sum()
}
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
