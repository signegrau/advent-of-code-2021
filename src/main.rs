use std::fs;

fn parse_file(path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(path).expect("Could not read file");

    contents
        .split('\n')
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

fn solve_day1() {
    let count_fn = |(prev, count), n: &i32| (*n, if n > &prev { count + 1 } else { count });
    let lines = parse_file("inputs/day1.txt");
    // Puzzle 1
    let count1: i32 = lines.iter().skip(1).fold((lines[0], 0), count_fn).1;

    println!("1st puzzle: {}", count1);

    // Puzzle 2
    let sums: Vec<i32> = lines.windows(3).map(|window| window.iter().sum()).collect();

    let count2 = sums.iter().skip(1).fold((sums[0], 0), count_fn).1;

    println!("2nd puzzle: {}", count2);
}

fn solve_day2() {
    let contents = fs::read_to_string("inputs/day2.txt").expect("Could not read file");

    let movements: Vec<_> = contents.split('\n').map(|l| {
        let arr: Vec<_> = l.split(' ').collect();
        match arr[..] {
            [c, d] => (c, d.parse::<i32>().unwrap()),
            _ => ("", 0)
        }
    }).collect();

    let puzzle1 = match movements.iter().fold((0, 0), |(z, y), movement| match movement {
        ("forward", d) => (z + d, y),
        ("down", d) => (z, y + d),
        ("up", d) => (z, y - d),
        _ => (z, y),
    }) {
        (z, y) => z * y,
    };

    let puzzle2 = match movements.iter().fold(
        (0, 0, 0),
        |(aim, z, y), movement| match movement {
            ("forward", d) => (aim, z + d, y + d * aim),
            ("down", d) => (aim + d, z, y),
            ("up", d) => (aim - d, z, y),
            _ => (aim, z, y),
        },
    ) {
        (_, z, y) => z * y,
    };

    println!("1st puzzle: {}", puzzle1);
    println!("2nd puzzle: {}", puzzle2);
}

fn main() {
    println!("❄️❄️❄️ Day one ❄️❄️❄️");
    solve_day1();

    println!("❄️❄️❄️ Day two ❄️❄️❄️");
    solve_day2();
}
