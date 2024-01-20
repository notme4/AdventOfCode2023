use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    strength: u8,
    label: &'a str,
}

impl<'a> Lens<'a> {
    fn build(strength: u8, label: &'a str) -> Option<Self> {
        if 1 <= strength && strength <= 9 {
            Some(Lens { strength, label })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct FocusBox<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> FocusBox<'a> {
    fn new() -> Self {
        FocusBox {
            lenses: Vec::with_capacity(9),
        }
    }

    fn insert(&mut self, lens: Lens<'a>) {
        for l in self.lenses.iter_mut() {
            if l.label == lens.label {
                return *l = lens;
            }
        }
        self.lenses.push(lens);
    }

    fn remove(&mut self, label: &str) {
        self.lenses.retain(|l| *l.label != *label);
    }

    fn focusing_power(&self, ind: u64) -> u64 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, l)| {
                dbg!((i, ind, l));
                <usize as TryInto<u64>>::try_into(i + 1).unwrap()
                    * (ind + 1)
                    * <u8 as Into<u64>>::into(l.strength)
            })
            .sum()
    }
}

fn hash(start_num: u8, s: &str) -> u8 {
    s.chars().fold(start_num, |u: u8, c: char| {
        u.wrapping_add(<char as TryInto<u8>>::try_into(c).expect("Ascii"))
            .wrapping_mul(17)
    })
}

fn get_label_and_other(s: &str) -> (&str, &str) {
    if let Some(pos) = s.chars().position(|c| c == '=') {
        (&s[..pos], &s[pos + 1..])
    } else {
        (&s[..s.len() - 1], &s[s.len() - 1..])
    }
}

pub fn func(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    let mut boxes: [FocusBox; 256] = std::array::from_fn(|_| FocusBox::new());
    for (label, s) in file_data.split(",").map(get_label_and_other) {
        let ind = hash(0, label) as usize;
        if s.chars().next() == Some('-') {
            boxes[ind].remove(label);
        } else {
            boxes[ind].insert(Lens::build(s.parse().expect("valid int"), label).expect("valid strength"));
        }
        //dbg!(boxes.iter().enumerate().filter(|(_, v)| !v.lenses.is_empty()).collect::<Vec<_>>());
    }
    boxes.iter().enumerate().map(|(ind, b)| b.focusing_power(ind.try_into().unwrap())).sum()
}
