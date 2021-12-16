use std::fmt;
use std::fs;
use std::str::FromStr;

use itertools::Itertools;

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

fn str_to_num<T>(s: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    s.to_string().trim().parse::<T>().unwrap()
}

fn str_to_numbers<T>(s: &str, delimiters: &[char]) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    s.split(&delimiters[..])
        .filter(|&c| c != "")
        .map(str_to_num::<T>)
        .collect::<Vec<T>>()
}

// Row major storage of data in a matrix.
#[derive(Debug)]
pub struct Matrix {
    data: Vec<i32>,
    shape: (usize, usize),
    stride: usize,
}

impl Matrix {
    // pub fn zeros(rows: usize, cols: usize) -> Self {
    //     Matrix {
    //         data: vec![0; rows * cols],
    //         shape: (rows, cols),
    //         stride: cols,
    //     }
    // }

    pub fn from_slice(rows: usize, cols: usize, numbers: &[i32]) -> Self {
        let mut m = Matrix {
            data: Vec::new(),
            shape: (rows, cols),
            stride: cols,
        };
        m.data.extend_from_slice(numbers);
        m
    }

    pub fn print(&self) {
        for row in 0..self.shape.0 {
            let start = row * self.stride;
            let end = (row + 1) * self.stride;
            for &x in &self.data[start..end] {
                print!("{: >3}", x);
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn at(&self, row: usize, col: usize) -> i32 {
        self.data[row * self.stride + col]
    }

    pub fn row(&self, row: usize) -> Vec<i32> {
        self.data[row * self.stride..row * self.stride + 4].to_vec()
    }

    pub fn col(&self, col: usize) -> Vec<i32> {
        let mut c = Vec::new();
        for row in 0..self.shape.0 {
            c.push(self.data[row * self.stride + col]);
        }
        c
    }
}

pub struct BoardState {
    id: i32,
    board: Matrix,
    row_count: Vec<usize>,
    col_count: Vec<usize>,
    bingo: bool,
}

pub struct BingoResult {
    board_id: i32,
    the_number: i32,
    winning_line: Vec<i32>,
}

impl BoardState {
    pub fn new(id: i32, s: &str) -> Self {
        let m = Matrix::from_slice(5, 5, &str_to_numbers(s, &[' ', '\n']));
        println!("Created board:");
        m.print();
        BoardState {
            id,
            board: m,
            row_count: vec![0; 5],
            col_count: vec![0; 5],
            bingo: false,
        }
    }

    fn is_bingo(&self) -> Option<(char, usize)> {
        let row = self.row_count.iter().position(|x| x >= &5);
        if let Some(row_index) = row {
            return Some(('r', row_index));
        }
        let col = self.row_count.iter().position(|x| x >= &5);
        if let Some(col_index) = col {
            return Some(('r', col_index));
        }
        None
    }

    fn parse_result(&self, res: (char, usize)) -> Vec<i32> {
        match res {
            ('c', idx) => self.board.row(idx),
            ('r', idx) => self.board.row(idx),
            (c, _) => {
                panic!("Wrong result {}", c);
            }
        }
    }

    pub fn update(&mut self, bingo_number: i32) -> Option<BingoResult> {
        if !self.bingo {
            let indices = self.board.data.iter().positions(|&x| x == bingo_number);
            for idx in indices {
                let col = idx % self.board.stride;
                let row = idx / self.board.stride;
                self.col_count[col] += 1;
                self.row_count[row] += 1;
            }
            // println!("{:?}", &self.col_count);
            // println!("{:?}", &self.row_count);

            if let Some(res) = self.is_bingo() {
                println!(
                    "BINGO {} !!! at board {} r/c {} index {}",
                    bingo_number, self.id, res.0, res.1
                );
                self.bingo = true;
                println!("result {:?}", self.parse_result(res));
                return Some(BingoResult {
                    board_id: self.id,
                    the_number: bingo_number,
                    winning_line: self.parse_result(res),
                });
            }
        }
        None
    }
}

fn main() {
    let text_blocks: Vec<_> = _TEST_INPUT.split("\n\n").collect();
    // let input = fs::read_to_string("data/day4.txt").unwrap();
    // let text_blocks: Vec<_> = input.split("\n\n").collect();

    let draws: Vec<i32> = str_to_numbers(&text_blocks[0], &[',']);

    let mut boards = Vec::<BoardState>::new();
    let mut id = 0;
    for s in &text_blocks[1..] {
        boards.push(BoardState::new(id, s));
        id += 1;
    }

    // let mut state = BoardState::new(&text_blocks[1]);
    let mut bingo = false;
    let mut res = BingoResult {
        board_id: -1,
        the_number: 0,
        winning_line: Vec::new(),
    };
    for lucky_number in draws {
        for state in boards.iter_mut() {
            match state.update(lucky_number) {
                Some(r) => {
                    res = r;
                    bingo = true;
                }
                None => (),
            }
            if bingo {
                break;
            }
        }
        if bingo {
            break;
        }
    }

    // assert_eq!(boards[2].board.col(2), vec![17, 15, 23, 13, 12]);

    let b = &boards[res.board_id as usize];

    println!("winning board:\n");
    b.board.print();

    let total_sum: i32 = b.board.data.iter().sum();
    let row_sum: i32 = res.winning_line.iter().sum();
    println!("Solution {}", (total_sum - row_sum) * res.the_number);
}
