use std::fs::read_to_string;
use std::path::Path;

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    return (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect();
}

fn tilt(platform: &mut Vec<char>) -> Vec<char> {
    let mut start = 0;
    let mut num_round = 0;
    for ind in 0..platform.len() {
        let c = platform[ind];
        match c {
            '.' => assert!(true),
            'O' => {
                num_round += 1;
                platform[ind] = '.';
            }
            '#' => {
                for i in 0..num_round {
                    platform[start + i] = 'O';
                }
                num_round = 0;
                start = ind + 1;
            }
            _ => panic!("invalid character"),
        }
    }
    for i in 0..num_round {
        platform[start + i] = 'O';
    }
    return platform.to_vec();
}

#[allow(dead_code)]
fn eprintln_through<T: std::fmt::Debug>(t: T) -> T {
    eprintln!("{:?}", t);
    return t;
}

fn cycle(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut platform = platform.clone().iter_mut().map(tilt).collect(); // N
    platform = transpose(&transpose(&platform).iter_mut().map(tilt).collect()); // W
    platform = platform
        .into_iter()
        .map(|v| v.into_iter().rev().collect::<Vec<char>>())
        .map(|mut v| tilt(&mut v))
        .map(|v| v.into_iter().rev().collect::<Vec<char>>())
        .collect(); // S
    platform = transpose(
        &transpose(&platform)
            .into_iter()
            .map(|v| v.into_iter().rev().collect::<Vec<char>>())
            .map(|mut v| tilt(&mut v))
            .map(|v| v.into_iter().rev().collect::<Vec<char>>())
            .collect(),
    ); // E
    return platform;
}

fn same_vec_vec<T>(v1: &Vec<Vec<T>>, v2: &Vec<Vec<T>>) -> bool
where
    T: PartialEq<T> + std::fmt::Display,
{
    for (i, (t1, t2)) in v1.iter().flatten().zip(v2.iter().flatten()).enumerate() {
        if i % v1.len() == 0 {
            eprintln!();
        }
        eprint!("{},{} ", *t1, *t2);
        if *t1 != *t2 {
            return false;
        }
    }
    return true;

    //for (v1_2, v2_2) in v1.iter().zip(v2) {
    //    for (t1, t2) in v1_2.iter().zip(v2_2) {
    //        if *t1 != *t2 {
    //            //eprintln!("{t1} {t2} {c} {r}");
    //            return false;
    //        }
    //    }
    //}
    //return true;
}

pub fn func(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    let platform = file_data
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let platform = &mut transpose(&platform);
    eprintln!(
        "\n{:#?}\n",
        platform
            .iter()
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<String>>()
    );
    let mut platform2 = vec![cycle(&platform)];
    for _ in 0..1_000_000_000 {
        eprintln!(
            "{:#?}",
            transpose(platform2.iter().last().unwrap())
                .iter()
                .map(|v| v.into_iter().collect::<String>())
                .collect::<Vec<String>>()
        );
        if platform2.iter().any(|p| same_vec_vec(platform, p)) {
            eprintln!("match");
            //break;
        }
        *platform = platform2.iter().last().unwrap().clone();
        platform2.push(cycle(platform));
    }
    eprintln!(
        "\n{:#?}\n",
        transpose(&platform)
            .iter()
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<String>>()
    );
    let start_loop = platform2
        .iter()
        .position(|p| same_vec_vec(platform, p))
        .unwrap()
        + 1;
    let iterations_mod = (1_000_000_000 - start_loop) % (platform2.len() - start_loop);
    eprintln!(
        "start: {start_loop}, mod: {iterations_mod}, len: {}",
        platform2.len()
    );

    todo!()
}
