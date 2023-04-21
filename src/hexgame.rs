use crate::DisjointSet;
use colored::Colorize;

#[derive(PartialEq, Clone, Copy)]
enum Color {
    Blank,
    Red,
    Blue
}

pub struct HexGame {
    logic_board: DisjointSet,
    game_board: Vec<Color>,
    pub size: usize,
    top_edge: usize,
    bottom_edge: usize,
    left_edge: usize,
    right_edge: usize
}

impl HexGame {
    pub fn new(_size: usize) -> Self {
        // # of nodes is the # of rows squared
        // add four for the top, bottom, left, and right edges.

        let logic: DisjointSet = DisjointSet::new((_size * _size) + 4);
        let mut game: Vec<Color> = Vec::new();

        for _ in 0..((_size * _size) + 4) {
            game.push(Color::Blank);
        }

        let top = game.len() - 4;
        let bot = game.len() - 3;
        let left = game.len() - 2;
        let right = game.len() - 1;

        println!("top: {}, bot: {}, left: {}, right: {}", top, bot, left, right);

        Self {
            logic_board: logic,
            game_board: game,
            size: _size,
            top_edge: top,
            bottom_edge: bot,
            left_edge: left,
            right_edge: right
        }
    }

    pub fn play_red(&mut self, position: usize, display_neighbors: bool) -> bool {
        // shift position by 1
        let position = position - 1;

        if position > (self.game_board.len() - 3) {
            panic!("Index out of bounds!");
        }

        let neighbors: Vec<usize> = self.get_neighbors(position, true);

        // print move
        if display_neighbors {
            print!("Cell {}: [ ", position + 1);
            for neighbor in &neighbors {
                print!("{} ", neighbor);
            }
            print!("]\n");
        }

        // if taken, do nothing and return false
        if self.is_occupied(position) { return false; }

        self.game_board[position] = Color::Red;

        // join the cell with its red neighbors
        for neighbor in &neighbors {
            if self.game_board[*neighbor] == Color::Red ||
               *neighbor >= self.game_board.len() - 4 {
                self.logic_board.union(position, *neighbor);
            }
        }

        println!("{:?}", self.logic_board.find(&self.top_edge));
        println!("{:?}", self.logic_board.find(&self.bottom_edge));

        // check for win condition
        return self.logic_board.find(&self.top_edge) 
            == self.logic_board.find(&self.bottom_edge);
    }

    pub fn play_blue(&mut self, position: usize, display_neighbors: bool) -> bool {
        // shift position by 1
        let position = position - 1;

        if position > (self.game_board.len() - 3) {
            panic!("Index out of bounds!");
        }

        let neighbors: Vec<usize> = self.get_neighbors(position, false);

        // print move
        if display_neighbors {
            print!("Cell {}: [ ", position + 1);
            for neighbor in &neighbors {
                print!("{} ", neighbor);
            }
            print!("]\n");
        }

        // if taken, do nothing and return false
        if self.is_occupied(position) { return false; }

        self.game_board[position] = Color::Blue;

        // join the cell with its blue neighbors
        for neighbor in &neighbors {
            if self.game_board[*neighbor] == Color::Blue ||
               *neighbor >= self.game_board.len() - 4 {
                self.logic_board.union(position, *neighbor);
            }
        }

        // check for win condition
        return self.logic_board.find(&self.left_edge) 
            == self.logic_board.find(&self.right_edge);
    }

    fn is_occupied(&self, position: usize) -> bool {
        return self.game_board[position] != Color::Blank;
    }

    fn get_neighbors(&self, position: usize, is_red: bool) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        // this is horrific

        // top edge
        if position < self.size {
            println!("top");
            if is_red { result.push(self.top_edge); }

            // left edge
            if position % self.size == 0 {
                if !is_red { result.push(self.left_edge); }
                result.push(position + 1);
                result.push(position + self.size);
            // right edge
            } else if (position + 1) % self.size == 0 {
                if !is_red { result.push(self.right_edge); }
                result.push(position - 1);
                result.push(position + (self.size - 1));
                result.push(position + self.size);
            // middle
            } else {
                result.push(position - 1);
                result.push(position + 1);
                result.push(position + (self.size - 1));
                result.push(position + self.size);
            }
        // bottom edge
        } else if position > (self.size - 1) * self.size - 1 {
            println!("bottom");
            if is_red { result.push(self.bottom_edge); }

            // left edge
            if position % self.size == 0 {
                if !is_red { result.push(self.left_edge); }
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
            // right edge
            } else if (position + 1) % self.size == 0 {
                if !is_red { result.push(self.right_edge); }
                result.push(position - 1);
                result.push(position - self.size);
            // middle
            } else {
                result.push(position - 1);
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
            }
        // middle
        } else {
            // left edge
            if position % self.size == 0 {
                if !is_red { result.push(self.left_edge); }
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
                result.push(position + self.size);
            // right edge
            } else if (position + 1) % self.size == 0 {
                if !is_red { result.push(self.right_edge); }
                result.push(position - 1);
                result.push(position - self.size);
                result.push(position + (self.size - 1));
                result.push(position + self.size);
            // middle
            } else {
                result.push(position - 1);
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
                result.push(position + (self.size - 1));
                result.push(position + self.size);
            }
        }

        return result;
    }
 
    pub fn print(&self) {
        // set cursor to row 1, col 1
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for i in 0..self.size {
            for _ in 0..i {
                print!(" ");
            }

            for j in (i * self.size)..((i * self.size) + self.size) {
                match self.game_board[j] {
                    Color::Red   => { print!("{}", "● ".bright_red()) },
                    Color::Blue  => { print!("{}", "● ".bright_blue()) },
                    Color::Blank => { print!("{}", "● ".bright_black()) }
                }
            }
            println!();
        }

        println!();
    }
}
