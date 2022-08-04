use std::collections::HashMap;

const EMPTY: char = ' ';
const X: char = 'X';
const O: char = 'O';

type Board = [[char; 3]; 3];
type Keys = [String; 8];

fn main() {
    test();
    let empty_board: Board = [[EMPTY; 3]; 3];
    let boards: Vec<Board> = all_boards(empty_board, X);
    let mut terminal_boards: HashMap<String, Board> = HashMap::new();
    for board in boards {
        if is_terminal(board) && !contains(&terminal_boards, board) {
            terminal_boards.insert(representation(board), board);
        }
    }
    println!("{} unique terminal boards found", terminal_boards.len());
    for (_, board) in terminal_boards {
        display(board);
    }
}

fn all_boards(board: Board, to_play: char) -> Vec<Board> {
    let to_play_next: char = if to_play == X { O } else { X };
    let mut boards: Vec<Board> = Vec::new();
    for x in 0..board.len() {
        let row = board[x];
        for y in 0..row.len() {
            if board[x][y] == EMPTY {
                let mut b: Board = board.clone();
                b[x][y] = to_play;
                boards.push(b);
                if !is_terminal(b) {
                    boards.append(&mut all_boards(b, to_play_next));
                }
            }
        }
    }
    return boards
}

fn display(board: Board) {
    for row in board {
        for square in row {
            print!("| {} ", square);
        }
        print!("|\n");
    }
    println!("\n")
}

fn is_terminal(board: Board) -> bool {
    for x in 0..board.len() {
        if board[x][0] != EMPTY && board[x][0] == board[x][1] && board[x][1] == board[x][2] {
            return true;
        }
    }
    for y in 0..board[0].len() {
        if board[0][y] != EMPTY && board[0][y] == board[1][y] && board[1][y] == board[2][y] {
            return true;
        }
    }
    if board[0][0] != EMPTY && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return true;
    }
    if board[0][2] != EMPTY && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return true;
    }
    for row in board {
        for square in row {
            if square == EMPTY {
                return false;
            }
        }
    }
    return true;
}

fn contains(boards: &HashMap<String, Board>, board: Board) -> bool {
    let keys: Keys = all_representations(board);
    for key in keys {
        if boards.contains_key(&key) {
            return true;
        }
    }
    return false;
}

fn all_representations(board: Board) -> Keys {
    let mut r: Keys = Default::default();
    r[0] = representation(board);
    r[1] = representation(rotate(board));
    r[2] = representation(rotate(rotate(board)));
    r[3] = representation(rotate(rotate(rotate(board))));
    r[4] = representation(transpose(board));
    r[5] = representation(transpose(rotate(board)));
    r[6] = representation(transpose(rotate(rotate(board))));
    r[7] = representation(transpose(rotate(rotate(rotate(board)))));
    return r;
}

fn representation(board: Board) -> String {
    let mut s = String::new();
    for row in board {
        for square in row {
            s.push(square);
        }
    }
    return s;
}

fn rotate(board: Board) -> Board {
    let mut b: Board = [[EMPTY, EMPTY, EMPTY]; 3];
    for x in 0..board.len() {
        let row = board[x];
        for y in 0..row.len() {
            b[x][y] = board[row.len() - y - 1][x];
        }
    }
    return b
}

fn transpose(board: Board) -> Board {
    let mut b: Board = [[EMPTY, EMPTY, EMPTY]; 3];
    for x in 0..board.len() {
        let row = board[x];
        for y in 0..row.len() {
            b[y][x] = board[x][y];
        }
    }
    return b;
}

fn test() {
    test_rotate();
    test_transpose();
}

fn test_rotate() {
    // XXX    _OX
    // OOO => _OX
    // ___    _OX
    let b: Board = [[X, X, X], [O, O, O], [EMPTY, EMPTY, EMPTY]];
    let rotated: Board = rotate(b);
    assert!(rotated[0] == [EMPTY, O, X]);
    assert!(rotated[1] == [EMPTY, O, X]);
    assert!(rotated[2] == [EMPTY, O, X]);
}

fn test_transpose() {
    // X_O    XO_
    // OX_ => _X_
    // __X    O_X
    let b: Board = [[X, EMPTY, O], [O, X, EMPTY], [EMPTY, EMPTY, X]];
    let transposed: Board = transpose(b);
    assert!(transposed == [[X, O, EMPTY], [EMPTY, X, EMPTY], [O, EMPTY, X]]);
}