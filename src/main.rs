mod rotations;
use bitvec::prelude::*;

type Board = [BitArr!(for 10, in u16, Lsb0); 40];

fn format_board(board: &Board, rows: usize) -> String {
    let mut s = String::new();
    for row in board.iter().take(rows).rev() {
        for b in row {
            match b.as_ref() {
                true => s.push('1'),
                false => s.push('0')
            };
        }
        s.push('\n');
    }
    s
}

fn main() {
    let mut board: Board = [bitarr!(u16, Lsb0; 0; 1); 40];
    board[0].set(0, true);
    println!("{:?}", board[0]);
    println!("{}", format_board(&board, 20));
}
