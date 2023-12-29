use std::fs::read_to_string;
use std::path::Path;

fn to_vec_u32(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|val| val.parse::<u32>().ok())
        .collect()
}

fn get_split_scratch_card(line: &str) -> (u32, Vec<u32>, Vec<u32>) {
    //eprintln!("original line: {}", line);
    let (winning_nums, elfs_nums) = line.split_once("|").unwrap();
    let (_, winning_nums) = winning_nums.split_once(":").unwrap();
    //eprintln!("winning_nums: {}", winning_nums);
    //eprintln!("elfs_nums: {}", elfs_nums);
    let winning_nums = to_vec_u32(winning_nums);
    let elfs_nums = to_vec_u32(elfs_nums);

    return (1, winning_nums, elfs_nums);
}

fn get_num_new_cards(card: &(u32, Vec<u32>, Vec<u32>)) -> u32 {
    let (num, winning, elfs) = card;
    elfs
            .iter()
            .map(|val| if winning.contains(val) { 1 } else { 0 })
            .sum()
}

fn get_score(mut cards: Vec<(u32, Vec<u32>, Vec<u32>)>) -> u32 {
    let mut result = 0;
    let mut ind = 0;
    while ind < cards.len() {
        let card = &cards[ind];
        let num_card = card.0;
        result += num_card;
        let mut num_new_cards = get_num_new_cards(card);
        while 0 < num_new_cards {
            cards[ind + usize::try_from(num_new_cards).unwrap()].0 += num_card;
            num_new_cards -= 1;
        }
        ind += 1;
    }
    return result;
}

pub fn scratch_cards(path: &Path) -> u32 {
    get_score(
        read_to_string(path)
            .unwrap()
            .lines()
            .map(get_split_scratch_card)
            .collect::<Vec<_>>(),
    )
}
