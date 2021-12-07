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

fn main() {
    println!("❄️❄️❄️ Day one ❄️❄️❄️");
    solve_day1();
}
