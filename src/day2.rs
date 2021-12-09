use std::fs;

static _TEST_INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

fn main() {
    let str_to_i32 = |s: &str| s.to_string().trim().parse::<i32>().unwrap();

    let mut depth = 0;
    let mut position = 0;

    // _TEST_INPUT
    fs::read_to_string("data/day2.txt")
        .unwrap()
        .lines()
        // Lazy but ugly?
        .map(|line| line
            .split(' ')
            .collect::<Vec<_>>())
        // Chunks chunk!
        // .flat_map(|line| line.split(' '))
        // .collect::<Vec<_>>()
        // .chunks(2)
        .map(|x| (x[0], str_to_i32(x[1])))
        .for_each(|(action, amount)| match action {
            "down" => depth += amount,
            "up" => depth -= amount,
            "forward" => position += amount,
            _ => (),
        });

    println!("depth: {}, horizontal position: {}", depth, position);
    println!("product: {}", depth * position);

    depth = 0;
    position = 0;
    let mut aim = 0;

    // _TEST_INPUT
    fs::read_to_string("data/day2.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|x| (x[0], str_to_i32(x[1])))
        .for_each(|(action, amount)| match action {
            "down" => aim += amount,
            "up" => aim -= amount,
            "forward" => {
                position += amount;
                depth += aim * amount
            }
            _ => (),
        });

    println!("depth: {}, horizontal position: {}", depth, position);
    println!("aim: {}", aim);
    println!("product: {}", depth * position);
}
