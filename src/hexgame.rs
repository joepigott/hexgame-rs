use crate::termcolor::*;
use crate::disjointset::DisjointSet;

#[derive(PartialEq, Clone, Copy)]
enum Color {
    Blank,
    Red,
    Blue,
}

pub struct HexGame {
    logic: DisjointSet,
    game: Vec<Color>,
    pub size: usize,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl HexGame {
    /// Returns a new HexGame instance with a `size * size` game board.
    pub fn new(size: usize) -> Self {
        // # of nodes is the # of rows squared
        // add four for the top, bottom, left, and right edges.

        let logic: DisjointSet = DisjointSet::new((size * size) + 4);
        let mut game: Vec<Color> = Vec::new();

        for _ in 0..((size * size) + 4) {
            game.push(Color::Blank);
        }

        let top = game.len() - 4;
        let bottom = game.len() - 3;
        let left = game.len() - 2;
        let right = game.len() - 1;

        Self {
            logic,
            game,
            size,
            top,
            bottom,
            left,
            right,
        }
    }

    /// Places a Red piece at the given position. Returns true if the win
    /// condition has been met.
    pub fn play_red(&mut self, position: usize) -> bool {
        // shift position by 1
        let position = position - 1;

        if position > (self.game.len() - 3) {
            panic!("Index out of bounds!");
        }

        let neighbors: Vec<usize> = self.get_neighbors(position, true);

        // if taken, do nothing and return false
        if self.is_occupied(position) {
            return false;
        }

        self.game[position] = Color::Red;

        // join the cell with its red neighbors, plus edges
        for neighbor in &neighbors {
            if self.game[*neighbor] == Color::Red || *neighbor >= self.game.len() - 4 {
                self.logic.union(position, *neighbor);
            }
        }

        // check for win condition
        return self.logic.find(&self.top) == self.logic.find(&self.bottom);
    }

    /// Places a Blue piece at the given positon. Returns true if the win
    /// condition has been met.
    pub fn play_blue(&mut self, position: usize) -> bool {
        // shift position by 1
        let position = position - 1;

        let neighbors: Vec<usize> = self.get_neighbors(position, false);

        // if taken, do nothing and return false
        if self.is_occupied(position) {
            return false;
        }

        self.game[position] = Color::Blue;

        // join the cell with its blue neighbors, plus edges
        for neighbor in &neighbors {
            if self.game[*neighbor] == Color::Blue || *neighbor >= self.game.len() - 4 {
                self.logic.union(position, *neighbor);
            }
        }

        // check for win condition
        return self.logic.find(&self.left) == self.logic.find(&self.right);
    }

    /// Returns true if the given position has already been played.
    fn is_occupied(&self, position: usize) -> bool {
        return self.game[position] != Color::Blank;
    }

    /// Returns the neighbors of a given position. If `is_red` is `true`, the
    /// top and bottom edges will be counted. If false, the left and right
    /// edges will be counted.
    fn get_neighbors(&self, position: usize, is_red: bool) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        // this is horrific

        // top edge
        if position < self.size {
            if is_red {
                result.push(self.top);
            }

            // left edge
            if position % self.size == 0 {
                if !is_red {
                    result.push(self.left);
                }
                result.push(position + 1);
                result.push(position + self.size);
            // right edge
            } else if (position + 1) % self.size == 0 {
                if !is_red {
                    result.push(self.right);
                }
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
            if is_red {
                result.push(self.bottom);
            }

            // left edge
            if position % self.size == 0 {
                if !is_red {
                    result.push(self.left);
                }
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
            // right edge
            } else if (position + 1) % self.size == 0 {
                if !is_red {
                    result.push(self.right);
                }
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
                if !is_red {
                    result.push(self.left);
                }
                result.push(position + 1);
                result.push(position - self.size);
                result.push(position - (self.size - 1));
                result.push(position + self.size);
            // right edge
            } else if (position + 1) % self.size == 0 {
                if !is_red {
                    result.push(self.right);
                }
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

    /// Prints the game board as a skewed 2-D array.
    pub fn print(&self) {
        // set cursor to row 1, col 1
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for i in 0..self.size {
            for _ in 0..i {
                print!(" ");
            }

            for j in (i * self.size)..((i * self.size) + self.size) {
                match self.game[j] {
                    Color::Red => {
                        print!("{}", Red("●"))
                    }
                    Color::Blue => {
                        print!("{}", Blue("●"))
                    }
                    Color::Blank => {
                        print!("{}", Gray("●"))
                    }
                }
            }
            println!();
        }
        println!();
    }

    /// Returns true if the move is within the bounds of the game board.
    pub fn is_valid_move(&self, position: usize) -> bool {
        return !(position > (self.size * self.size) || position == 0);
    }
}
