use std::fs::File;
use std::io::Read;
use std::{io, vec};


mod inputs;
fn _get_input() -> Result<String, io::Error> {
    inputs::get_input("day4")
}


#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Board {
    marked: Vec<Point>,
    grid: Vec<Vec<i32>>,
}

#[derive(Debug)]
struct Bingo {
    curr_pos: i32,
    curr_number: i32,
    draw_numbers: Vec<i32>,
    boards: Vec<Board>,
    winning_board_index : i32
}

fn parse_board(input: Vec<String>) -> Board {
    let mut board: Board = Board {
        marked: vec![],
        grid: vec![],
    };

    for l in input {
        match l.replace("  ", " ").trim() {
            "" => {}
            li => board
                .grid
                .push(li.split(" ").map(|n| n.parse::<i32>().unwrap()).collect()),
        }
    }
    return board;
}

fn parse_input(input: String) -> Bingo {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let mut bingo = Bingo {
        curr_pos: 0,
        curr_number: 0,
        draw_numbers: vec![],
        boards: vec![],
        winning_board_index: -1
    };
    let draw_numbers: Vec<i32> = lines
        .iter()
        .next()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    bingo.draw_numbers = draw_numbers;

    for i in 0..lines.len() / 6 {
        //println!("i: {i}, length:{}", lines.len());
        let board_input = lines.get(((i * 6) + 1)..((i + 1) * 6) + 1).unwrap();

        //println!("board_input {board_input:?}");
        let board = parse_board(board_input.to_vec());

        //println!("board {board:?}");
        bingo.add_board(board);
    }

    return bingo;
}

impl Board {
    fn mark_number(&mut self, num: i32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.grid[row][col] == num {
                    self.marked.push(Point { row: row, col: col });
                }
            }
        }
    }

    fn check_win( &self) -> bool {
        for i in 0..5 {
            if self.marked.iter().filter(|p| p.row == i).count() == 5 {
                return true;
            }
            
            if self.marked.iter().filter(|p| p.col == i).count() == 5 {
                return true;
            }
        }
        return false;
    }

    fn is_marked(&mut self, row: i32, col:i32) -> bool{
        for point in self.marked.iter_mut() {
            if point.row == row as usize && point.col == col as usize{
                return true;
            }
        }
        return false;
    }

    fn get_sum_of_unmarked_nums(&mut self ) -> i32 {

        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if ! self.is_marked(row, col){
                    sum += self.grid[row as usize][col as usize];
                }
            }
        }
        return sum;
    }
}

impl Bingo {
    fn print(&self) {
        println!("Curr state: {self:?}")
    }

    fn draw_number(&mut self) {
        self.curr_number = self.draw_numbers[self.curr_pos as usize];

        for board in self.boards.iter_mut() {
            board.mark_number(self.curr_number);
        }

        self.curr_pos += 1;
    }

    fn is_game_over(&mut self) -> bool{
        let mut idx = 0;
        for board in self.boards.iter(){
            if board.check_win() {
                self.winning_board_index = idx;
                return true;
            }
            idx += 1;
        }
        return false;
    }
    fn add_board(&mut self, board: Board) {
        self.boards.push(board);
    }
}

fn main() {
    let mut bingo: Bingo = parse_input(_get_input().unwrap());

    while !bingo.is_game_over() {
        
        bingo.draw_number();

    
    }

    let unmarked_sum = (bingo.boards[bingo.winning_board_index as usize]).get_sum_of_unmarked_nums();
    let winning_num = bingo.curr_number;
    let answer = unmarked_sum * winning_num;

   println!("Winning number: {winning_num}, unmarked_sum: {unmarked_sum}\nAnswer: {answer}")
}
