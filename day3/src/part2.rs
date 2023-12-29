use std::collections::HashSet;
use std::fs::read_to_string;
use std::option::Option;
use std::path::Path;

fn char_is_digit(line: &str, index: usize) -> bool {
    if let Some(c) = line.chars().nth(index) {
        if c.is_digit(10) {
            return true;
        }
    }
    return false;
}

fn to_number(s: &str) -> u32 {
    let mut result = 0;
    for c in s.chars() {
        result = result * 10 + c.to_digit(10).unwrap();
    }
    return result;
}

fn get_number(line: &str, index: usize) -> Option<(usize, usize)> {
    if !char_is_digit(line, index) {
        return None;
    }
    let mut left = index;
    let mut right = index;
    while left != 0 && char_is_digit(line, left - 1) {
        left -= 1;
    }
    while char_is_digit(line, right) {
        right += 1;
    }
    return Some((left, right));
}

fn get_part_numbers(
    lines: &Vec<&str>,
    line_num: usize,
    index: usize,
) -> HashSet<(usize, (usize, usize))> {
    let prev_line = if line_num > 0 {
        lines[line_num - 1]
    } else {
        ""
    };
    let current_line = lines[line_num];
    let next_line = if line_num < lines.len() - 1 {
        lines[line_num + 1]
    } else {
        ""
    };

    let mut nums: HashSet<(usize, (usize, usize))> = HashSet::new();
    if let Some(slice) = get_number(prev_line, index - 1) {
        nums.insert((line_num - 1, slice));
    }
    if let Some(slice) = get_number(prev_line, index) {
        nums.insert((line_num - 1, slice));
    }
    if let Some(slice) = get_number(prev_line, index + 1) {
        nums.insert((line_num - 1, slice));
    }
    if let Some(slice) = get_number(current_line, index - 1) {
        nums.insert((line_num, slice));
    }
    if let Some(slice) = get_number(current_line, index + 1) {
        nums.insert((line_num, slice));
    }
    if let Some(slice) = get_number(next_line, index - 1) {
        nums.insert((line_num + 1, slice));
    }
    if let Some(slice) = get_number(next_line, index) {
        nums.insert((line_num + 1, slice));
    }
    if let Some(slice) = get_number(next_line, index + 1) {
        nums.insert((line_num + 1, slice));
    }
    println!("nums start");
    for i in nums.iter() {
        println!("({}, ({}, {}))", i.0, i.1 .0, i.1 .1);
    }
    println!("nums end");
    return nums;
}

fn get_gear(lines: &Vec<&str>, line_num: usize, char_pos: usize) -> Option<(u32, u32)> {
    let mut nums = get_part_numbers(lines, line_num, char_pos);
    if nums.len() == 2 {
        let a = nums.iter().next().unwrap().clone();
        nums.remove(&a);
        let b = nums.iter().next().unwrap().clone();
        nums.remove(&b);
        let a_slice = &lines[a.0][(a.1 .0)..(a.1 .1)];
        let b_slice = &lines[b.0][(b.1 .0)..(b.1 .1)];
        let result = (to_number(a_slice), to_number(b_slice));
        //println!(
        //    " at: {},{} -> gear: ({}, {})",
        //    line_num + 1,
        //    char_pos + 1,
        //    result.0,
        //    result.1
        //);
        return Some(result);
    }
    return None;
}

fn find_gears(lines: Vec<&str>) -> Vec<(u32, u32)> {
    let mut gears: Vec<(u32, u32)> = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let mut line_slice = &line[..];
        while let Some(i) = line_slice.find('*') {
            if let Some(gear) = get_gear(&lines, line_num, i) {
                gears.push(gear);
            }

            line_slice = &line_slice[i + 1..]
        }
    }
    return gears;
}

pub fn gear_ratios(path: &Path) -> u32 {
    let data = read_to_string(path).unwrap();
    let lines: Vec<_> = data.lines().collect::<Vec<_>>();
    return find_gears(lines).into_iter().map(|x| x.0 * x.1).sum();
    // return sum_gear_ratios(lines);
}
