use nalgebra as na;


use advent::str_to_numbers;

static _TEST_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

pub struct BoardState {
    row_count: Vec<usize>,
    col_count: Vec<usize>,
    bingo: bool,
    mask : na::Matrix5::<i32>,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            row_count: vec![0; 5],
            col_count: vec![0; 5],
            bingo: false,
            mask : na::Matrix5::<i32>::repeat(0),
        }
    }

    fn update_internals(&mut self, idx : &(usize, usize)){
        self.mask[*idx] = 1;
        self.row_count[idx.0] += 1;
        self.col_count[idx.1] += 1;
    }

    pub fn update(&mut self, bingo_number: i32, m : &na::Matrix5::<i32>) -> Vec<(usize, usize)> {
        let mut found : Vec<(usize, usize)> = Vec::new();
        let selector = |i, j, x| {
            if x == bingo_number {
                found.push((i, j));
                true
            }
            else {
                false
            }
        };
        let _ = m.map_with_location(selector);
        found.iter().for_each(|x| self.update_internals(x));
        found
    }

}


fn main() {

    let text_blocks: Vec<_> = _TEST_INPUT.split("\n\n").collect();
    let elements: Vec<i32> = str_to_numbers(text_blocks[1], &[' ', '\n']);

    let m = na::Matrix5::<i32>::from_row_slice(&elements);

    // mask[(0, 0)] = 1;
    // mask[(0, 1)] = 1;

    // println!("dot: {}", m.dot(&mask));

    println!("{}", m);

    let mut state = BoardState::new();

    let _ = state.update(11, &m);
    let _ = state.update(2, &m);
    let _ = state.update(4, &m);
    let _ = state.update(16, &m);
    let _ = state.update(18, &m);
    let _ = state.update(200, &m);
    let _ = state.update(15, &m);

    println!("{}", state.mask);


    // println!("{}", m.row(0).sum());
}