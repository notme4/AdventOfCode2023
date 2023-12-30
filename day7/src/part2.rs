use itertools::Itertools;
use std::fs::read_to_string;
use std::path::Path;

struct CamelCard {
    hand: [u8; 5],
    value: u32, // better hands = lower values
    bid: u64,
}

impl std::fmt::Debug for CamelCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return f
            .debug_struct("CampelCard")
            .field("hand", &std::str::from_utf8(&self.hand).unwrap())
            .field("value", &format_args!("{:#X}", self.value))
            .field("bid", &self.bid)
            .finish();
    }
}

// 5 of a kind
//   55555
//   54321
// 4 of a kind
//   44441
//   43211
// full house 3 of a kind + 2 of a kind
//   33322
//   32211
// 3 of a kind
//   33311
//   32111
// 2 pair 2 of a kind + 2 of a kind
//   22221
//   22111
// 2 of a kind
//   22111
//   21111
// high card
//   11111
//   11111

// bottom 20 bits are guarenteed to be 0
fn hand_type_value(hand: &[u8; 5]) -> u32 {
    let mut arr = [0, 0, 0, 0, 0];
    for (ind, card) in hand.iter().enumerate() {
        if *card == 74
        /* 'J' */
        {
            arr[ind] = 6;
            continue;
        }
        for c2 in hand {
            arr[ind] += if card == c2 { 1 } else { 0 };
        }
    }
    arr.sort();
    match arr {
        [6, 6, 6, 6, 6] => return 0 << 20,
        [1, 6, 6, 6, 6] => return 0 << 20,
        [2, 2, 6, 6, 6] => return 0 << 20,
        [3, 3, 3, 6, 6] => return 0 << 20,
        [4, 4, 4, 4, 6] => return 0 << 20,
        [5, 5, 5, 5, 5] => return 0 << 20,

        [1, 1, 6, 6, 6] => return 1 << 20,
        [1, 2, 2, 6, 6] => return 1 << 20,
        [1, 3, 3, 3, 6] => return 1 << 20,
        [1, 4, 4, 4, 4] => return 1 << 20,

        [2, 2, 2, 2, 6] => return 2 << 20,
        [2, 2, 3, 3, 3] => return 2 << 20,

        [1, 1, 2, 2, 6] => return 3 << 20,
        [1, 1, 1, 6, 6] => return 3 << 20,
        [1, 1, 3, 3, 3] => return 3 << 20,

        [1, 2, 2, 2, 2] => return 4 << 20,

        [1, 1, 1, 1, 6] => return 5 << 20,
        [1, 1, 1, 2, 2] => return 5 << 20,

        [1, 1, 1, 1, 1] => return 6 << 20,

        _ => panic!("invalid arr: {:?}, {:?}", arr, hand),
    }
}

fn hand_value(hand: &[u8; 5]) -> u32 {
    let mut offset = 0;
    let mut result = hand_type_value(hand);
    for card in hand.iter().rev() {
        match card {
            65 /* 'A' */ => result += 0  << offset,
            75 /* 'K' */ => result += 1  << offset,
            81 /* 'Q' */ => result += 2  << offset,
            74 /* 'J' */ => result += 13 << offset,
            84 /* 'T' */ => result += 4  << offset,
            57 /* '9' */ => result += 5  << offset,
            56 /* '8' */ => result += 6  << offset,
            55 /* '7' */ => result += 7  << offset,
            54 /* '6' */ => result += 8  << offset,
            53 /* '5' */ => result += 9  << offset,
            52 /* '4' */ => result += 10 << offset,
            51 /* '3' */ => result += 11 << offset,
            50 /* '2' */ => result += 12 << offset,
            _ => panic!("invalid card value: {}", card),
        };
        eprintln!("offset: {}", offset);
        offset += 4;
    }

    return result;
}

fn create_camel_card(line: &str) -> CamelCard {
    let (hand, bid) = line.split_at(5);
    let hand = hand.trim().as_bytes();
    let bid = bid.trim().parse::<u64>().unwrap();
    if hand.len() != 5 {
        panic!("hand is not 5 chars: {}", hand.len());
    }
    let hand = [hand[0], hand[1], hand[2], hand[3], hand[4]];
    return CamelCard {
        hand: hand,
        value: hand_value(&hand),
        bid: bid,
    };
}

pub fn func(path: &Path) -> u64 {
    return read_to_string(path)
        .unwrap()
        .lines()
        .map(create_camel_card)
        .sorted_by_key(|cc| cc.value)
        .rev()
        .enumerate()
        .map(|(ind, cc)| {
            eprintln!("{} {:?}", ind, cc);
            return ((ind as u64) + 1) * cc.bid;
        })
        .sum();
}
