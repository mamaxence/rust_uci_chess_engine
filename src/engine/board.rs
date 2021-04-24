use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::string::ParseError;
use crate::engine::piece::Piece;

#[derive(Debug, Copy, Clone)]
pub struct BoardParseError;

#[derive(Debug, Copy, Clone)]
pub struct Board{
    /// board internal representation
    /// indices start for the bottom left and got to left:
    /// ie :
    ///     8  56 57 58 59 60 61 62 63
    ///     7  48 49 50 51 52 53 54 55
    ///     6  40 41 42 43 44 45 46 47
    ///     5  32 33 34 35 36 37 38 39
    ///     4  24 25 26 27 28 29 30 31
    ///     3  16 17 18 19 22 21 22 23
    ///     2  8  9  10 11 12 13 14 15
    ///     1  0  1  2  3  4  5  6  7
    ///        a  b  c  d  e  f  g  h
    board: [Option<Piece>; 64],
}
impl Index<usize> for Board{
    type Output = Option<Piece>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}
impl IndexMut<usize> for Board{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[index]
    }
}
impl fmt::Display for Board{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for l in (0..8).rev(){
            str.push_str(format!("{} ", l+1).as_str());
            for c in 0..8{
                match self[c + l*8] {
                    Some(piece) => str.push_str(piece.to_string().as_str()),
                    None => str.push('.')
                }
            }
            str.push('\n');
        }
        str.push_str("  abcdefgh");
        write!(f, "{}", str)
    }
}
/// Board can be loaded from a fen representation:
/// (https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
impl FromStr for Board{
    type Err = ParseError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new_empty_board();

        let split_fen: Vec<&str> = fen.split(' ').collect();
        for (l, line) in split_fen[0].split('/').enumerate(){
            let mut col: usize = 0;
            for car in line.chars(){
                if let Some(num) = car.to_digit(10) {
                    col += num as usize
                } else{
                    board[col + 8*(7-l as usize)] = Some(format!("{}", car).parse().unwrap());
                    col += 1;
                }
            }
        }

        Ok(board)
    }
}
impl Board{
    /// Create a new board with no pieces.
    pub fn new_empty_board() -> Self{
        Board{board: [None; 64]}
    }

    /// Create a new  board with starting position.
    pub fn new_board() -> Self{
        Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn new_from_fen(fen: &str) -> Self{
        fen.parse().unwrap()
    }
}

#[cfg(test)]
mod tests{
    use crate::engine::board::Board;

    #[test]
    fn debug_board(){
        let empty_board = Board::new_empty_board();
        println!("{:?}", empty_board);
        println!("{}", empty_board);
        let board = Board::new_board();
        println!("{:?}", board);
        println!("{}", board);
    }

    #[test]
    fn fen_load(){
        let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut expected = "\
8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ........
3 ........
2 PPPPPPPP
1 RNBQKBNR
  abcdefgh";
        println!("{}", Board::new_from_fen(fen));
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);

        fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        expected = "\
8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ....P...
3 ........
2 PPPP.PPP
1 RNBQKBNR
  abcdefgh";
        println!("{}", Board::new_from_fen(fen));
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);

        fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        expected = "\
8 rnbqkbnr
7 pp.ppppp
6 ........
5 ..p.....
4 ....P...
3 ........
2 PPPP.PPP
1 RNBQKBNR
  abcdefgh";
        println!("{}", Board::new_from_fen(fen));
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);

        fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        expected = "\
8 rnbqkbnr
7 pp.ppppp
6 ........
5 ..p.....
4 ....P...
3 .....N..
2 PPPP.PPP
1 RNBQKB.R
  abcdefgh";
        println!("{}", Board::new_from_fen(fen));
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);

    fen = "r3r1k1/pp3nPp/1b1p1B2/1q1P1N2/8/P4Q2/1P3PK1/R6R";
        expected = "\
8 r...r.k.
7 pp...nPp
6 .b.p.B..
5 .q.P.N..
4 ........
3 P....Q..
2 .P...PK.
1 R......R
  abcdefgh";
        println!("{}", Board::new_from_fen(fen));
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);
    }
}
