use std::fs;

static _TEST_INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

fn str_to_i32(s: char) -> i32 {
    s.to_string().trim().parse::<i32>().unwrap()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.to_string().chars().map(|c| str_to_i32(c)).collect()
}

fn add_vectors(va: &[i32], vb: &[i32]) -> Vec<i32> {
    va.iter().zip(vb.iter()).map(|(a, b)| a + b).collect()
}

fn bin2dec(b: &[i32]) -> i32 {
    b.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x * 2_i32.pow(i as u32))
}

fn main() {
    let input = fs::read_to_string("data/day3.txt").unwrap();

    let num_digits = input.lines().next().unwrap().len();
    let num_lines = input.lines().count() as i32;

    let column_sums = input
        .lines()
        .map(parse_line)
        .fold(vec![0; num_digits], |acc, x| add_vectors(&acc, &x));

    let gamma = column_sums
        .iter()
        .map(|&x| x > (num_lines - x))
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    let epsilon = column_sums
        .iter()
        .map(|&x| x < (num_lines - x))
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    println!("solution: {}", bin2dec(&gamma) * bin2dec(&epsilon));
}
