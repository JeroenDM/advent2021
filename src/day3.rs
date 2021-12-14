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

fn parse_matrix(s: &String) -> Vec<Vec<i32>> {
    s.lines().map(parse_line).collect()
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

fn colwise_sum(m: &Vec<Vec<i32>>) -> Vec<i32> {
    let num_digits = m[0].len();
    m.iter()
        .fold(vec![0; num_digits], |acc, x| add_vectors(&acc, &x))
}

fn reduce_column<F>(lines: Vec<Vec<i32>>, col_idx: usize, criteria: F) -> i32
where
    F: Fn(&i32, i32) -> bool + Copy,
{
    let num_digits = lines[0].len();

    let column_sums = lines
        .iter()
        .fold(vec![0; num_digits], |acc, x| add_vectors(&acc, &x));

    let select_bit = column_sums
        .iter()
        .map(|x| criteria(x, lines.len() as i32))
        // .map(|&x| x >= (lines.len() as i32 - x))
        .map(|x| x as i32)
        .nth(col_idx)
        .unwrap();

    let new_lines = lines
        .into_iter()
        .filter(|x| x[col_idx] == select_bit)
        .collect::<Vec<_>>();

    if new_lines.len() > 1 && col_idx + 1 < num_digits {
        reduce_column(new_lines, col_idx + 1, criteria)
    } else {
        return bin2dec(&new_lines[0]);
    }
}

fn main() {
    // let input = _TEST_INPUT;
    let input = fs::read_to_string("data/day3.txt").unwrap();

    let m = parse_matrix(&input);

    let num_lines = input.lines().count() as i32;

    let gamma = colwise_sum(&m)
        .iter()
        .map(|&x| x > (num_lines - x))
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    let epsilon = colwise_sum(&m)
        .iter()
        .map(|&x| x < (num_lines - x))
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    println!("Solution 1: {}", bin2dec(&gamma) * bin2dec(&epsilon));

    let oxygen_rating = reduce_column(m.clone(), 0, |&x, total| x >= (total - x));
    let co2_rating = reduce_column(m.clone(), 0, |&x, total| x < (total - x));

    println!("Solution 2: {}", oxygen_rating * co2_rating);
}
