use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use crate::engine::piece::{Piece, Color, PieceParseError};
use crate::engine::piece::Color::White;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone)]
pub struct ParseCastleError;

/// Represent the available castle move in a game
#[derive(Debug, Copy, Clone)]
pub struct Castle{
    white_king: bool,
    white_queen: bool,
    black_king: bool,
    black_queen: bool
}
impl FromStr for Castle{
    type Err = ParseCastleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut castle = Castle{white_king: false, white_queen: false, black_king: false, black_queen: false};
        if s == "-"{
            Ok(castle)
        }else{
            for car in s.chars(){
                match car {
                    'K' => castle.white_king = true,
                    'Q' => castle.white_queen = true,
                    'k' => castle.black_king= true,
                    'q' => castle.black_queen = true,
                    _ => {return Err(ParseCastleError)}
                };
            }
            Ok(castle)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ParseCaseError;

/// Represent a case of the chessboard
#[derive(Debug, Copy, Clone)]
struct Case(usize);
impl FromStr for Case{
    type Err = ParseCaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars:Vec<char> = s.chars().collect();
        if chars.len() != 2{
            return Err(ParseCaseError)
        }

        let col = match chars[0] {  // match col
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {return Err(ParseCaseError);}
        };
        let line = match chars[1] { // match line
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => {return Err(ParseCaseError);}
        };

        Ok(Case(col + line*8))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoardParseError;
impl From<ParseCastleError> for BoardParseError{
    fn from(_: ParseCastleError) -> Self {
        BoardParseError
    }
}
impl From<ParseCaseError> for BoardParseError{
    fn from(_: ParseCaseError) -> Self {
        BoardParseError
    }
}
impl From<ParseIntError> for BoardParseError{
    fn from(_: ParseIntError) -> Self {
        BoardParseError
    }
}
impl From<PieceParseError> for BoardParseError{
    fn from(_: PieceParseError) -> Self {
        BoardParseError
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board{
    /// board internal representation
    /// indices start for the bottom left and got to left:
    /// ie :
    /// ```
    ///     8  56 57 58 59 60 61 62 63
    ///     7  48 49 50 51 52 53 54 55
    ///     6  40 41 42 43 44 45 46 47
    ///     5  32 33 34 35 36 37 38 39
    ///     4  24 25 26 27 28 29 30 31
    ///     3  16 17 18 19 22 21 22 23
    ///     2  8  9  10 11 12 13 14 15
    ///     1  0  1  2  3  4  5  6  7
    ///        a  b  c  d  e  f  g  h
    /// ```
    board: [Option<Piece>; 64],
    /// The next side to play
    side: Color,
    /// Castle available
    castle: Castle,
    /// Available 'en passant' if any
    en_passant: Option<Case>,
    /// number of half move since last capture of pawn advance
    halfmove: u32,
    /// number of move in the game
    moves: u32,
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
    type Err = BoardParseError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new_empty_board();
        let split_fen: Vec<&str> = fen.split(' ').collect();

        if split_fen.len() != 6{
            return Err(BoardParseError);
        }

        // parse fen position
        for (l, line) in split_fen[0].split('/').enumerate(){
            let mut col: usize = 0;
            for car in line.chars(){
                if let Some(num) = car.to_digit(10) {
                    col += num as usize
                } else{
                    board[col + 8*(7-l as usize)] = Some(format!("{}", car).parse()?);
                    col += 1;
                }
            }
        }

        // parse side
        match split_fen[1]{
            "w" => board.side = White,
            "b" => board.side = White,
            _ => return Err(BoardParseError)
        }

        // parse castle
        board.castle = split_fen[2].parse()?;

        // parse 'en passant'
        if split_fen[3] == "-"{
            board.en_passant = None
        } else{
            board.en_passant = Some(split_fen[3].parse()?)
        }

        // parse halfmove
        board.halfmove = split_fen[4].parse()?;

        // parse move
        board.moves = split_fen[5].parse()?;

        Ok(board)
    }
}
impl Board{
    /// Create a new board with no pieces.
    pub fn new_empty_board() -> Self{
        Board{board: [None; 64],
            side:White,
            castle: "QKqk".parse().unwrap(),
            en_passant:None,
            halfmove:0,
            moves:0}
    }

    /// Create a new  board with starting position.
    pub fn new_board() -> Self{
        Board::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    /// Create a new board from a fen repressentation:
    /// (https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
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

    fen = "r3r1k1/pp3nPp/1b1p1B2/1q1P1N2/8/P4Q2/1P3PK1/R6R b KQkq - 1 2";
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
