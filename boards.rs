use std::collections::HashMap;

const EMPTY: char = ' ';
const X: char = 'X';
const O: char = 'O';

type Boards = HashMap<String, Board>;
type Board = [[char; 3]; 3];
type Representations = [String; 8];

trait BoardMethods {
    fn display(&self) -> String;
    fn is_terminal(&self) -> bool;
    fn representation(&self) -> String;
    fn all_representations(&self) -> Representations;
    fn rotate(&self) -> Board;
    fn transpose(&self) -> Board;
}

trait BoardsMethods {
    fn contains(&self, b: Board) -> bool;
    fn insert_board(&mut self, b: Board);
}

fn main() {
    let terminal_boards = terminal_boards();
    let n = terminal_boards.len();
    println!("{} unique terminal boards found\n", n);
    for board in terminal_boards {
        println!("{}", board.display());
    }
    println!("{} unique terminal boards found\n", n);
}

fn terminal_boards() -> Vec<Board> {
    let empty_board = [[EMPTY; 3]; 3];
    let mut final_boards = HashMap::new();
    let boards = all_boards(empty_board, X);
    for board in boards {
        if board.is_terminal() && !final_boards.contains(board) {
            final_boards.insert_board(board);
        }
    }
    final_boards.into_values().collect()
}

fn all_boards(board: Board, to_play: char) -> Vec<Board> {
    let to_play_next = if to_play == X { O } else { X };
    let mut boards = Vec::new();
    for x in 0..board.len() {
        let row = board[x];
        for y in 0..row.len() {
            if board[x][y] != EMPTY {
                continue;
            }
            let mut b = board.clone();
            b[x][y] = to_play;
            boards.push(b);
            if !b.is_terminal() {
                boards.append(&mut all_boards(b, to_play_next));
            }
        }
    }
    boards
}

impl BoardMethods for Board {
    fn display(&self) -> String {
        let mut s = String::new();
        for row in self {
            for square in row {
                s.push('|');
                s.push(*square);
            }
            s.push_str("|\n");
        }
        s
    }

    fn is_terminal(&self) -> bool {
        for i in 0..3 {
            if self[i][0] != EMPTY && self[i][0] == self[i][1] && self[i][1] == self[i][2] {
                return true;
            }
            if self[0][i] != EMPTY && self[0][i] == self[1][i] && self[1][i] == self[2][i] {
                return true;
            }
        }
        if self[0][0] != EMPTY && self[0][0] == self[1][1] && self[1][1] == self[2][2] {
            return true;
        }
        if self[0][2] != EMPTY && self[0][2] == self[1][1] && self[1][1] == self[2][0] {
            return true;
        }
        for row in self {
            for square in row {
                if *square == EMPTY {
                    return false;
                }
            }
        }
        true
    }

    fn all_representations(&self) -> Representations {
        let mut r: Representations = Default::default();
        r[0] = self.representation();
        r[1] = self.transpose().representation();
        let mut b = self.rotate();
        r[2] = b.representation();
        r[3] = b.transpose().representation();
        b = b.rotate();
        r[4] = b.representation();
        r[5] = b.transpose().representation();
        b = b.rotate();
        r[6] = b.representation();
        r[7] = b.transpose().representation();
        r
    }

    fn representation(&self) -> String {
        let mut s = String::new();
        for row in self {
            for square in row {
                s.push(*square);
            }
        }
        s
    }

    fn rotate(&self) -> Board {
        let mut b = [[EMPTY; 3]; 3];
        for x in 0..self.len() {
            let row = self[x];
            for y in 0..row.len() {
                b[x][y] = self[row.len() - y - 1][x];
            }
        }
        b
    }

    fn transpose(&self) -> Board {
        let mut b = [[EMPTY; 3]; 3];
        for x in 0..self.len() {
            let row = self[x];
            for y in 0..row.len() {
                b[y][x] = self[x][y];
            }
        }
        b
    }
}

impl BoardsMethods for Boards {
    fn contains(&self, board: Board) -> bool {
        for repr in board.all_representations() {
            if self.contains_key(&repr) {
                return true;
            }
        }
        false
    }

    fn insert_board(&mut self, b: Board) {
        self.insert(b.representation(), b);
    }
}


#[test]
fn test_rotate() {
    // |X|X|X|      | |O|X|
    // |O|O|O|  =>  | |O|X|
    // | | | |      | |O|X|
    let b = [[X, X, X], [O, O, O], [EMPTY, EMPTY, EMPTY]];
    let rotated = b.rotate();
    assert_eq!(rotated[0], [EMPTY, O, X]);
    assert_eq!(rotated[1], [EMPTY, O, X]);
    assert_eq!(rotated[2], [EMPTY, O, X]);
}

#[test]
fn test_transpose() {
    // |X| |O|      |X|O| |
    // |O|X| |  =>  | |X| |
    // | | |X|      |O| |X|
    let b = [[X, EMPTY, O], [O, X, EMPTY], [EMPTY, EMPTY, X]];
    let transposed = b.transpose();
    assert_eq!(transposed, [[X, O, EMPTY], [EMPTY, X, EMPTY], [O, EMPTY, X]]);
}

#[test]
fn test_representation() {
    let b = [[X, X, X], [X, X, X], [X, X, X]];
    let expected = "XXXXXXXXX";
    for actual in b.all_representations() {
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_terminal_boards() {
    let tbs = terminal_boards();
    assert_eq!(138, tbs.len());
}
