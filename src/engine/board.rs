use std::fmt;
use std::fmt::{Formatter, Display};
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use crate::engine::piece::{Piece, Color, PieceParseError};
use crate::engine::piece::Color::{White, Black};
use std::num::ParseIntError;
use crate::engine::moves::{Move, MoveKind};
use crate::engine::piece::PieceKind::{Knight, Bishop, Rook, Queen};

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
impl Display for Castle{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !(self.black_queen && self.black_king && self.white_queen && self.white_king) {
            write!(f, "-")
        }else{
            let mut res = String::new();
            if self.white_king {
                res.push('K');
            }
            if self.white_queen {
                res.push('Q');
            }
            if self.black_king {
                res.push('k');
            }
            if self.black_queen {
                res.push('q');
            }
            write!(f,"{}",  res)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Dir{
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Cav1,
    Cav2,
    Cav4,
    Cav5,
    Cav7,
    Cav8,
    Cav10,
    Cav11
}
#[derive(Debug, Copy, Clone)]
pub struct ParseCaseError;

/// Represent a case of the chessboard
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Case(usize);
impl Case{
    pub fn new(place: usize) -> Case{
        Case(place)
    }

    pub fn new_from_str(place: &str) -> Case{
        place.parse().unwrap()
    }

    /// get the line of the case
    pub fn get_line(&self) -> usize{
        return (self.0)/8
    }

    /// get the column of the case
    pub fn get_column(&self) -> usize{
        return (self.0)%8
    }
    /// return the neighbour of the case given dir and dist
    pub fn get_neighbour(&self, dir: Dir, distance: usize) -> Option<Case>{
        match dir {
            Dir::Up =>
                if self.get_line() < (8-distance) {
                    Some(Case(self.0+8*distance))
                } else {None},
            Dir::UpRight =>
                if self.get_line() < (8-distance) && self.get_column() < (8-distance) {
                    Some(Case(self.0+9*distance))
                } else {None}
            Dir::Right =>
                if self.get_column() < (8-distance) {
                    Some(Case(self.0+1*distance))}
                else {None}
            Dir::DownRight =>
                if self.get_line() >= (distance) && self.get_column() < (8-distance) {
                    Some(Case(self.0-7*distance))}
                else {None}
            Dir::Down =>
                if self.get_line() >= (distance) {
                    Some(Case(self.0-8*distance))}
                else {None}
            Dir::DownLeft =>
                if self.get_line() >= (distance) && self.get_column() >= (distance){
                    Some(Case(self.0-9*distance))}
                else {None}
            Dir::Left =>
                if self.get_column() >= (distance){
                    Some(Case(self.0-1*distance))}
                else {None}
            Dir::UpLeft =>
                if self.get_line() < (8-distance) && self.get_column() >= (distance){
                    Some(Case(self.0+7*distance))}
                else {None},
            Dir::Cav1 =>
                if self.get_line() < 7 && self.get_column() < 8 {
                    Some(Case(self.0+17))
                } else {None}
            Dir::Cav2 =>
                if self.get_line() < 8 && self.get_column() < 7 {
                    Some(Case(self.0+10))
                } else {None}
            Dir::Cav4 =>
                if self.get_line() >= 1 && self.get_column() < 7 {
                    Some(Case(self.0-6))
                } else {None}
            Dir::Cav5 =>
                if self.get_line() >= 2 && self.get_column() < 8 {
                    Some(Case(self.0-15))
                } else {None}
            Dir::Cav7 =>
                if self.get_line() >= 2 && self.get_column() >= 1 {
                    Some(Case(self.0-17))
                } else {None}
            Dir::Cav8 =>
                if self.get_line() >= 1 && self.get_column() >= 2 {
                    Some(Case(self.0-10))
                } else {None}
            Dir::Cav10 =>
                if self.get_line() < 8 && self.get_column() >= 2 {
                    Some(Case(self.0-10))
                } else {None}
            Dir::Cav11 =>
                if self.get_line() < 7 && self.get_column() >= 1 {
                    Some(Case(self.0-10))
                } else {None}
        }
    }
}
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
impl fmt::Display for Case{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let line = self.get_line() + 1;
        let col = self.get_column();
        let col_names = ["a", "b", "c" ,"d" ,"e", "f", "g", "h"];
        write!(f, "{}{}", col_names[col], line)
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
    pub(crate) en_passant: Option<Case>,
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
impl Index<&Case> for Board{
    type Output = Option<Piece>;

    fn index(&self, case: &Case) -> &Self::Output {
        &self.board[case.0]
    }
}
impl IndexMut<usize> for Board{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[index]
    }
}
impl IndexMut<&Case> for Board{
    fn index_mut(&mut self, case: &Case) -> &mut Self::Output {
        &mut self.board[case.0]
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
            "b" => board.side = Black,
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
            moves:1}
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

    pub fn to_fen(&self) -> String{
        let mut res = String::new();
        for line in (0..8).rev(){
            let mut n_empty = 0;
            for col in 0..8 {
                match self[col + 8*line]{
                    Some(piece) =>{
                        if n_empty!= 0 {
                            res.push_str(&format!("{}{}", n_empty, piece));
                            n_empty = 0;
                        } else {
                            res.push_str(&piece.to_string())
                        }
                    },
                    None => n_empty += 1,
                }
            }
            match n_empty {
                0 => res.push_str(r"/"),
                val => res.push_str(&format!("{}/",val))
            }
        }
        res.pop(); // remove last '/'

        res.push_str(&format!(" {} {} ", self.side, self.castle));
        if let Some(case) = self.en_passant{
            res.push_str(&case.to_string());
        }else {
            res.push('-');
        }
        res.push_str(&format!(" {} {}", self.halfmove, self.moves));
        res
    }

    /// Apply a move and return a new board
    pub fn apply_move(&self, mv: &Move) -> Self{
        let mut new = self.clone();
        match mv.get_kind() {
            MoveKind::DoublePawnPush => {
                new[&mv.to] = new[&mv.from];
                new[&mv.from] = None;
                new.en_passant = match mv.from.get_line() {
                    1 => mv.from.get_neighbour(Dir::Up, 1),
                    6 => mv.from.get_neighbour(Dir::Down, 1),
                    _ => {panic!("invalid mv {}", mv); Some(mv.from)}
                }
            }
            MoveKind::KingCastle => {
                new[&mv.to] = new[&mv.from]; // move the king
                // move the tower
                let rock_target = &mv.to.get_neighbour(Dir::Left, 1).unwrap();
                let rock_source = &mv.to.get_neighbour(Dir::Right, 1).unwrap();
                new[rock_target] = new[rock_source];
                new[&mv.from] = None;
                new[rock_source] = None;
            }
            MoveKind::QueenCastle => {
                new[&mv.to] = new[&mv.from]; // move the king
                // move the tower
                let rock_target = &mv.to.get_neighbour(Dir::Right, 1).unwrap();
                let rock_source = &mv.to.get_neighbour(Dir::Left, 2).unwrap();
                new[rock_target] = new[rock_source];
                new[&mv.from] = None;
                new[rock_source] = None;
            }
            MoveKind::EnPassantCapture => {
                new[&mv.to] = new[&mv.from];
                new[&mv.from] = None;
                // remove the taken pawn
                match mv.to.get_line() {
                    2 => new[&mv.to.get_neighbour(Dir::Up, 1).unwrap()] = None,
                    5 => new[&mv.to.get_neighbour(Dir::Down, 1).unwrap()] = None,
                    _ => {panic!("invalid mv {}", mv)}
                }
            }
            MoveKind::KnightPromotion => {
                new[&mv.to] = Some(Piece{kind: Knight, color:new[&mv.from].unwrap().color});
                new[&mv.from] = None;
            }
            MoveKind::BishopPromotion => {
                new[&mv.to] = Some(Piece{kind: Bishop, color:new[&mv.from].unwrap().color});
                new[&mv.from] = None;
            }
            MoveKind::RookPromotion => {
                new[&mv.to] = Some(Piece{kind: Rook, color:new[&mv.from].unwrap().color});
                new[&mv.from] = None;
            }
            MoveKind::QueenPromotion => {
                new[&mv.to] = Some(Piece{kind: Queen, color:new[&mv.from].unwrap().color});
                new[&mv.from] = None;
            }
            MoveKind::KnightCapturePromotion => {
                new[&mv.to] = Some(Piece{kind: Knight, color:new[&mv.from].unwrap().color});
                new[&mv.from] = None;
            }
            MoveKind::BishopCapturePromotion => {
                new[&mv.to] = Some(Piece{kind: Bishop, color:new[&mv.from].unwrap().color});
                new[&mv.from] = None;
            }
            MoveKind::RookCapturePromotion => {
                new[&mv.to] = Some(Piece{kind: Rook, color:new[&mv.from].unwrap().color});
                new[&mv.from] = None;
            }
            MoveKind::QueenCapturePromotion => {
                new[&mv.to] = Some(Piece{kind: Queen, color:new[&mv.from].unwrap().color});
                new[&mv.from] = None;
            }
            _ => { // Default are Quiet ant simple capture
                new[&mv.to] = new[&mv.from];
                new[&mv.from] = None;
            },
        }
        match mv.is_capture(){
            true => new.halfmove = 0,
            false => new.halfmove += 1
        };
        match self.side {
            Color::White => new.side = Color::Black,
            Color::Black => {new.side = Color::White; new.moves += 1}
        }
        new
    }
}

#[cfg(test)]
mod tests{
    use crate::engine::board::{Board, Case};
    use crate::engine::moves::{Move, MoveKind};

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
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);
        assert_eq!(Board::new_from_fen(fen).to_fen(), fen);

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
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);
        assert_eq!(Board::new_from_fen(fen).to_fen(), fen);

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
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);
        assert_eq!(Board::new_from_fen(fen).to_fen(), fen);

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
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);
        assert_eq!(Board::new_from_fen(fen).to_fen(), fen);

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
        assert_eq!(Board::new_from_fen(fen).to_string(), expected);
        assert_eq!(Board::new_from_fen(fen).to_fen(), fen);
    }

    #[test]
    fn test_move_simple(){
        // see https://lichess.org/editor/2p1k2r/p2P2P1/8/8/4Pp2/8/1P6/R3K3_w_-_-_0_1
        let mut board = Board::new_from_fen("2p1k2r/p2P2P1/8/8/4Pp2/8/1P6/R3K3 w - e3 0 1");
        println!("{}", board);

        board = board.apply_move(&Move::new_on_board("f4f3", &board))
            .apply_move(&Move::new_on_board("b2b4", &board));
        //assert_eq!(board.en_passant, Case::new_from_str("b3"));
        board =board.apply_move(&Move::new_on_board("a7a5", &board));
//        assert_eq!(board.en_passant, Case::new_from_str("a6"));

        println!("{}", board);

        // assert_eq!(Move::new_on_board("e8g8", &board).get_kind(), MoveKind::KingCastle);
        // assert_eq!(Move::new_on_board("e1c1", &board).get_kind(), MoveKind::QueenCastle);
        // assert_eq!(Move::new_on_board("a1a7", &board).get_kind(), MoveKind::SimpleCapture);
        // assert_eq!(Move::new_on_board("f4e3", &board).get_kind(), MoveKind::EnPassantCapture);
        // assert_eq!(Move::new_on_board("g7g8n", &board).get_kind(), MoveKind::KnightPromotion);
        // assert_eq!(Move::new_on_board("g7g8b", &board).get_kind(), MoveKind::BishopPromotion);
        // assert_eq!(Move::new_on_board("g7g8r", &board).get_kind(), MoveKind::RookPromotion);
        // assert_eq!(Move::new_on_board("g7g8q", &board).get_kind(), MoveKind::QueenPromotion);
        // assert_eq!(Move::new_on_board("d7c8n", &board).get_kind(), MoveKind::KnightCapturePromotion);
        // assert_eq!(Move::new_on_board("d7c8b", &board).get_kind(), MoveKind::BishopCapturePromotion);
        // assert_eq!(Move::new_on_board("d7c8r", &board).get_kind(), MoveKind::RookCapturePromotion);
        // assert_eq!(Move::new_on_board("d7c8q", &board).get_kind(), MoveKind::QueenCapturePromotion);
    }
}
