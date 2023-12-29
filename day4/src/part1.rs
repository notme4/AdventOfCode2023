use std::fs::read_to_string;
use std::path::Path;

fn to_vec_u32(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|val| val.parse::<u32>().ok())
        .collect()
}

fn get_split_scratch_card(line: &str) -> (Vec<u32>, Vec<u32>) {
    //eprintln!("original line: {}", line);
    let (winning_nums, elfs_nums) = line.split_once("|").unwrap();
    let (_, winning_nums) = winning_nums.split_once(":").unwrap();
    //eprintln!("winning_nums: {}", winning_nums);
    //eprintln!("elfs_nums: {}", elfs_nums);
    let winning_nums = to_vec_u32(winning_nums);
    let elfs_nums = to_vec_u32(elfs_nums);

    return (winning_nums, elfs_nums);
}

fn get_score(card: (Vec<u32>, Vec<u32>)) -> u32 {
    let (winning, elfs) = card;
    let s: i8 = -1
        + elfs
            .iter()
            .map(|val| if winning.contains(val) { 1 } else { 0 })
            .sum::<i8>();
    return if s == -1 { 0 } else { 1 << s };
}

pub fn scratch_cards(path: &Path) -> u32 {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(get_split_scratch_card)
        .map(get_score)
        .sum()
}
