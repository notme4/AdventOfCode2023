use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

fn is_symbol(c: &u8) -> bool {
    c.is_ascii_punctuation() && *c != ('.' as u8)
}

fn find_part_numbers_line(
    prev_line: &str,
    current_line: &str,
    next_line: &str,
    line_num: usize,
) -> u32 {
    if line_num == 0 {
        return 0;
    }
    let mut sum: u32 = 0;
    let mut num: u32 = 0;
    let mut b = false;
    for (i, c) in current_line.as_bytes().iter().enumerate() {
        // 48 = '0', 57 = '9'
        if 48 <= *c && *c <= 57 {
            num = num * 10 + (*c as u32) - 48; // 48 = '0'
            if b {
                continue;
            }
            let prev_chars = prev_line.as_bytes();
            let current_chars = current_line.as_bytes();
            let next_chars = next_line.as_bytes();
            if line_num > 1 && i > 1 && is_symbol(&prev_chars[i - 1]) {
                println!("symbol is: {}", (prev_chars[i - 1] as char));
                b = true;
            } else if line_num > 1 && is_symbol(&prev_chars[i]) {
                println!("symbol is: {}", (prev_chars[i] as char));
                b = true;
            } else if line_num > 1 && i < prev_chars.len() - 1 && is_symbol(&prev_chars[i + 1]) {
                println!("symbol is: {}", (prev_chars[i + 1] as char));
                b = true;
            } else if i > 0 && is_symbol(&current_chars[i - 1]) {
                println!("symbol is: {}", (current_chars[i - 1] as char));
                b = true;
            } else if i < current_chars.len() - 1 && is_symbol(&current_chars[i + 1]) {
                println!("symbol is: {}", (current_chars[i + 1] as char));
                b = true;
            } else if next_line != "" && i > 0 && is_symbol(&next_chars[i - 1]) {
                println!("symbol is: {}", (next_chars[i - 1] as char));
                b = true;
            } else if next_line != "" && is_symbol(&next_chars[i]) {
                println!("symbol is: {}", (next_chars[i] as char));
                b = true;
            } else if next_line != "" && i < next_chars.len() - 1 && is_symbol(&next_chars[i + 1]) {
                println!("symbol is: {}", (next_chars[i + 1] as char));
                b = true;
            }
            continue;
        }
        if b {
            b = false;
            println!("number w/ part is: {}", num);
            sum += num;
        }
        num = 0;
    }
    if num != 0 && b {
        sum += num;
    }
    return sum;
}

fn find_part_numbers(lines: Lines) -> u32 {
    let mut prev_line = "";
    let mut current_line = "";
    let mut i: usize = 0;
    let mut sum = 0;
    for (ii, next_line) in lines.enumerate() {
        i = ii;
        sum += find_part_numbers_line(prev_line, current_line, next_line, i);
        prev_line = current_line;
        current_line = next_line;
    }
    sum += find_part_numbers_line(prev_line, current_line, "", i + 1);
    return sum;
}

pub fn gear_ratios(path: &Path) -> u32 {
    let data = read_to_string(path).unwrap();
    let lines = data.lines();
    return find_part_numbers(lines);
}
