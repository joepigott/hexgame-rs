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

        if position > (self.game_board.len() - 4) {
            panic!("Index out of bounds!");
        }

        let neighbors: Vec<usize> = self.get_neighbors(position, true);

        // print move
        if display_neighbors {
            print!("Cell {}: [ ", position + 1);
            for neighbor in &neighbors {
                print!("{} ", neighbor + 1);
            }
            print!("]\n");
        }

        // if taken, do nothing and return false
        if self.is_occupied(position) { return false; }

        self.game_board[position] = Color::Red;

        // if the cell is on the top or bottom edges, join it with those:
        if position < self.size {
            self.logic_board.union(position, self.top_edge);
        } else if position > (self.game_board.len() - 4) - self.size {
            self.logic_board.union(position, self.bottom_edge);
        }

        // join the cell with its red neighbors
        for neighbor in &neighbors {
            if self.game_board[*neighbor] == Color::Red {
                self.logic_board.union(position, *neighbor);
            }
        }

        // check for win condition
        return self.logic_board.find(&self.top_edge) 
            == self.logic_board.find(&self.bottom_edge);
    }

    pub fn play_blue(&mut self, position: usize, display_neighbors: bool) -> bool {
        // shift position by 1
        let position = position - 1;

        if position > (self.game_board.len() - 4) {
            panic!("Index out of bounds!");
        }

        let neighbors: Vec<usize> = self.get_neighbors(position, false);

        // print move
        if display_neighbors {
            print!("Cell {}: [ ", position + 1);
            for neighbor in &neighbors {
                print!("{} ", neighbor + 1);
            }
            print!("]\n");
        }

        // if taken, do nothing and return false
        if self.is_occupied(position) { return false; }

        self.game_board[position] = Color::Blue;

        // if the cell is on the left or right edges, join it with those:
        if position % self.size == 0 {
            self.logic_board.union(position, self.left_edge);
        } else if (position + 1) % self.size == 0 {
            self.logic_board.union(position, self.right_edge);
        }

        // join the cell with its blue neighbors
        for neighbor in &neighbors {
            if self.game_board[*neighbor] == Color::Blue {
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

        if position < self.size {
            if is_red { result.push(self.top_edge); }

            if position % self.size == 0 {
                if !is_red { result.push(self.left_edge); }
                result.push(position + 1);
                result.push(position + self.size);
            } else if (position + 1) % self.size == 0 {
                if !is_red { result.push(self.right_edge); }
                result.push(position - 1);
                result.push(position + (self.size - 1));
                result.push(position + self.size);
            } else {
                result.push(position - 1);
                result.push(position + 1);
                result.push(position + (self.size - 1));
                result.push(position + self.size);
            }
        } else if position > (self.game_board.len() - 4) - self.size {
            if is_red { result.push(self.bottom_edge); }

            if position % self.size == 0 {
                if !is_red { result.push(self.left_edge); }
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
            } else if (position + 1) % self.size == 0 {
                if !is_red { result.push(self.right_edge); }
                result.push(position - 1);
                result.push(position - self.size);
            } else {
                result.push(position - 1);
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
            }
        } else {
            if position % self.size == 0 {
                if !is_red { result.push(self.left_edge); }
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
                result.push(position + self.size);
            } else if (position + 1) % self.size == 0 {
                if !is_red { result.push(self.right_edge); }
                result.push(position - 1);
                result.push(position - self.size);
                result.push(position + (self.size - 1));
                result.push(position + self.size);
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
        println!();

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
