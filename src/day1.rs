use std::fs;

static _TEST_INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263"#;

fn main() {
    let str_to_i32 = |s: &str| s.to_string().trim().parse::<i32>().unwrap();

    let result_1 = _TEST_INPUT
    // let result_1 = fs::read_to_string("data/day1.txt")
    //     .unwrap()
        .lines()
        .map(str_to_i32)
        .collect::<Vec<_>>() // How can I do lazy windows??
        .windows(2)
        .map(|x| x[0] < x[1])
        .fold(0, |acc, x| acc + x as i32);

    println!("solution: {}", result_1);

    // let result_2 = _TEST_INPUT
    let result_2 = fs::read_to_string("data/day1.txt")
        .unwrap()
        .lines()
        .map(str_to_i32)
        .collect::<Vec<_>>()
        .windows(3)
        .map(|a| a[0] + a[1] + a[2])
        .collect::<Vec<_>>()
        .windows(2)
        .map(|a| a[0] < a[1])
        .fold(0, |acc, x| acc + x as i32);

    println!("solution: {}", result_2);
}
