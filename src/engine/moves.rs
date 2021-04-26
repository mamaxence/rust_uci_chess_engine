use crate::engine::board::{Case, ParseCaseError, Board};
use crate::engine::moves::SimpleKind::{Quiet, DoublePawnPush, KingCastle, QueenCastle};
use crate::engine::moves::CaptureKind::{Simple, EnPassant};
use crate::engine::moves::PromotionKind::{Knight, Bishop, Rook, Queen};
use crate::engine::moves::MoveKind::{SimpleCapture, KnightCapturePromotion, BishopCapturePromotion, RookCapturePromotion, QueenCapturePromotion, KnightPromotion, BishopPromotion, RookPromotion, QueenPromotion, EnPassantCapture};
use std::fmt;
use std::str::FromStr;
use crate::engine::piece::{Piece, PieceKind};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MoveKind{
    Quiet,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    SimpleCapture,
    EnPassantCapture,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightCapturePromotion,
    BishopCapturePromotion,
    RookCapturePromotion,
    QueenCapturePromotion,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
enum PromotionKind{
    Knight,
    Bishop,
    Rook,
    Queen,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
enum SimpleKind{
    Quiet,
    DoublePawnPush,
    KingCastle,
    QueenCastle
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
enum CaptureKind{
    Simple,
    EnPassant,
}

#[repr(C)]
#[derive(Copy, Clone)]
union SpecialFlag{
    simple: SimpleKind,
    promotion: PromotionKind,
    capture: CaptureKind,
}

/// Special flags for move
#[derive(Copy, Clone)]
struct MoveFlags{
    capture: bool,
    promotion: bool,
    kind: SpecialFlag,
}
impl From<MoveKind> for MoveFlags{
    fn from(kind: MoveKind) -> Self {
        match kind{
            MoveKind::Quiet =>
                MoveFlags{capture: false, promotion: false, kind:SpecialFlag{simple:Quiet}},
            MoveKind::DoublePawnPush =>
                MoveFlags{capture: false, promotion: false, kind:SpecialFlag{simple:DoublePawnPush}},
            MoveKind::KingCastle =>
                MoveFlags{capture: false, promotion: false, kind:SpecialFlag{simple:KingCastle}},
            MoveKind::QueenCastle =>
                MoveFlags{capture: false, promotion: false, kind:SpecialFlag{simple:QueenCastle}},
            MoveKind::SimpleCapture =>
                MoveFlags{capture: true, promotion: false, kind:SpecialFlag{capture:Simple}},
            MoveKind::EnPassantCapture =>
                MoveFlags{capture: true, promotion: false, kind:SpecialFlag{capture:EnPassant}},
            MoveKind::KnightPromotion =>
                MoveFlags{capture: false, promotion: true, kind:SpecialFlag{promotion:Knight}},
            MoveKind::BishopPromotion =>
                MoveFlags{capture: false, promotion: true, kind:SpecialFlag{promotion:Bishop}},
            MoveKind::RookPromotion =>
                MoveFlags{capture: false, promotion: true, kind:SpecialFlag{promotion:Rook}},
            MoveKind::QueenPromotion =>
                MoveFlags{capture: false, promotion: true, kind:SpecialFlag{promotion:Queen}},
            MoveKind::KnightCapturePromotion =>
                MoveFlags{capture: true, promotion: true, kind:SpecialFlag{promotion:Knight}},
            MoveKind::BishopCapturePromotion =>
                MoveFlags{capture: true, promotion: true, kind:SpecialFlag{promotion:Bishop}},
            MoveKind::RookCapturePromotion =>
                MoveFlags{capture: true, promotion: true, kind:SpecialFlag{promotion:Rook}},
            MoveKind::QueenCapturePromotion =>
                MoveFlags{capture: true, promotion: true, kind:SpecialFlag{promotion:Queen}},
        }
    }
}
impl Into<MoveKind> for MoveFlags{
    fn into(self) -> MoveKind {
        match self{
            MoveFlags {capture: false, promotion: false, kind} =>
            unsafe {
                match kind.simple {
                    Quiet => MoveKind::Quiet,
                    DoublePawnPush => MoveKind::DoublePawnPush,
                    KingCastle => MoveKind::KingCastle,
                    QueenCastle => MoveKind::QueenCastle,
                }
            }
            MoveFlags {capture: true, promotion: false, kind} =>
            unsafe {
                match kind.capture {
                    Simple => SimpleCapture,
                    EnPassant => EnPassantCapture,
                }
            }
            MoveFlags {capture: false, promotion: true, kind} =>
            unsafe {
                match kind.promotion {
                    Knight => KnightPromotion,
                    Bishop => BishopPromotion,
                    Rook => RookPromotion,
                    Queen => QueenPromotion,
                }
            }
            MoveFlags {capture: true, promotion: true, kind} =>
            unsafe {
                match kind.promotion{
                    Knight => KnightCapturePromotion,
                    Bishop => BishopCapturePromotion,
                    Rook => RookCapturePromotion,
                    Queen => QueenCapturePromotion
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MoveParseError;
impl From<ParseCaseError> for MoveParseError{
    fn from(_: ParseCaseError) -> Self {
        MoveParseError
    }
}

/// Represent a move for a chessboard
pub struct Move{
    pub from: Case,
    pub to: Case,
    flags: MoveFlags,
}
impl Move{

    /// Create a new move
    pub fn new(from: Case, to: Case, kind: MoveKind) -> Self{
        Move{from, to, flags: kind.into()}
    }

    /// Create a new move from an uci move text representation and a board.
    /// The board is needed for setting the move metadata
    pub fn new_on_board(str: &str, board: &Board) -> Self{
        let mut mv: Move = str.parse().unwrap();
        mv.set_kind(mv.get_kind_on_board(board));
        mv
    }

    pub fn is_capture(&self) -> bool{
        self.flags.capture
    }
    pub fn is_promotion(&self) -> bool{
        self.flags.promotion
    }
    pub fn get_kind(&self) -> MoveKind{
        self.flags.into()
    }
    pub fn set_kind(&mut self, kind: MoveKind){
        self.flags = kind.into();
    }


    /// Get the kind of a move given only its start and end position and its promotion kind if any
    /// This function assume the move is valid
    pub fn get_kind_on_board(&self, board: &Board) -> MoveKind{
        if self.is_promotion_on_board(board){
            if self.is_capture_on_board(board) {
                match self.get_kind() {
                    MoveKind::QueenPromotion => MoveKind::QueenCapturePromotion,
                    MoveKind::RookPromotion => MoveKind::RookCapturePromotion,
                    MoveKind::KnightPromotion => MoveKind::KnightCapturePromotion,
                    MoveKind::BishopPromotion => MoveKind::BishopCapturePromotion,
                    _ => MoveKind::Quiet // should not append
                }
            }
            else {
                self.get_kind()
            }
        }else if self.is_capture_on_board(board){
            MoveKind::SimpleCapture
        }else if self.is_en_passant_on_board(board) {
            MoveKind::EnPassantCapture
        } else if self.is_king_rock_on_board(board){
            MoveKind::KingCastle
        } else if self.is_queen_rock_on_board(board){
            MoveKind::QueenCastle
        } else if self.is_double_pawn_on_board(board){
            MoveKind::DoublePawnPush
        } else{
            MoveKind::Quiet
        }
    }

    /// Is the move a capture ? // todo (check en passant)
    fn is_capture_on_board(&self, board: &Board) -> bool{
        match board[&self.to] {
            None => false ,
            Some(_) => true
        }
    }

    /// Is the move a promotion ?
    fn is_promotion_on_board(&self, board: &Board) -> bool{
        match board[&self.from] {
            None => false,
            Some(Piece{kind: PieceKind::Pawn, color:_}) => self.to.get_line() == 0 || self.to.get_line() == 7,
            Some(Piece{kind:_, color:_}) => false,
        }
    }

    /// Is move a rock
    fn is_queen_rock_on_board(&self, board: &Board) -> bool{
        if let Some(Piece{kind: PieceKind::King, color:_}) = board[&self.from]{
            match (self.from.get_line(), self.from.get_column(), self.to.get_line(), self.to.get_column()) {
                (0, 4, 0, 2) => true, // white queen rock
                (7, 4, 7, 2) => true, // back queen rock
                (_, _, _, _) => false
            }
        }else {
            false
        }
    }

    /// Is move king rock
    fn is_king_rock_on_board(&self, board: &Board) -> bool{
        if let Some(Piece{kind: PieceKind::King, color:_}) = board[&self.from]{
            match (self.from.get_line(), self.from.get_column(), self.to.get_line(), self.to.get_column()) {
                (0, 4, 0, 6) => true, // white king rock
                (7, 4, 7, 6) => true, // back king rock
                (_, _, _, _) => false
            }
        }else {
            false
        }
    }


    /// Check if move is 'en passant'
    fn is_en_passant_on_board(&self, board: &Board) -> bool{
        if let Some(Piece{kind: PieceKind::Pawn, color:_}) = board[&self.from]{
            match board.en_passant{
                None => false,
                Some(case) => case == self.to
            }
        }else {
            false
        }
    }

    /// Check if move is a double pawn push
    fn is_double_pawn_on_board(&self, board: &Board) -> bool{
        if let Some(Piece{kind: PieceKind::Pawn, color:_}) = board[&self.from]{
            match (self.from.get_line(), self.to.get_line()) {
                (1, 3) => true,
                (6, 4) => true,
                (_, _) => false
            }
        }else {
            false
        }
    }
}
impl fmt::Display for Move{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}: {:?}", self.from, self.to, self.get_kind())
    }
}
impl FromStr for Move{
    type Err = MoveParseError;

    /// Create a move from a string. Be carefull to add the move kind with the board
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from: Case = s[0..2].parse()?;
        let to: Case = s[2..4].parse()?;
        if s.len() == 5{
            match s.chars().collect::<Vec<char>>()[4] {
                'q' => Ok(Move::new(from, to, MoveKind::QueenPromotion)),
                'n' => Ok(Move::new(from, to, MoveKind::KnightPromotion)),
                'b' => Ok(Move::new(from, to, MoveKind::BishopPromotion)),
                'r' => Ok(Move::new(from, to, MoveKind::RookPromotion)),
                _ => Err(MoveParseError)
            }
        } else{
            Ok(Move::new(from, to, MoveKind::Quiet))
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::engine::moves::{Move, SimpleKind, MoveKind};
    use crate::engine::board::{Case, Board};
    use crate::engine::moves::MoveKind::DoublePawnPush;

    #[test]
    fn test_print_move() {
        let m1 = Move::new(Case::new_from_str("e2"), Case::new_from_str("e4"), DoublePawnPush);
        println!("{}", m1)
    }

    #[test]
    fn test_move_creation_from_txt(){
        // see https://lichess.org/editor/2p1k2r/p2P2P1/8/8/4Pp2/8/1P6/R3K3_w_-_-_0_1
        let board = Board::new_from_fen("2p1k2r/p2P2P1/8/8/4Pp2/8/1P6/R3K3 w - e3 0 1");
        assert_eq!(Move::new_on_board("f4f3", &board).get_kind(), MoveKind::Quiet);
        assert_eq!(Move::new_on_board("b2b4", &board).get_kind(), MoveKind::DoublePawnPush);
        assert_eq!(Move::new_on_board("a7a5", &board).get_kind(), MoveKind::DoublePawnPush);
        assert_eq!(Move::new_on_board("e8g8", &board).get_kind(), MoveKind::KingCastle);
        assert_eq!(Move::new_on_board("e1c1", &board).get_kind(), MoveKind::QueenCastle);
        assert_eq!(Move::new_on_board("a1a7", &board).get_kind(), MoveKind::SimpleCapture);
        assert_eq!(Move::new_on_board("f4e3", &board).get_kind(), MoveKind::EnPassantCapture);
        assert_eq!(Move::new_on_board("g7g8n", &board).get_kind(), MoveKind::KnightPromotion);
        assert_eq!(Move::new_on_board("g7g8b", &board).get_kind(), MoveKind::BishopPromotion);
        assert_eq!(Move::new_on_board("g7g8r", &board).get_kind(), MoveKind::RookPromotion);
        assert_eq!(Move::new_on_board("g7g8q", &board).get_kind(), MoveKind::QueenPromotion);
        assert_eq!(Move::new_on_board("d7c8n", &board).get_kind(), MoveKind::KnightCapturePromotion);
        assert_eq!(Move::new_on_board("d7c8b", &board).get_kind(), MoveKind::BishopCapturePromotion);
        assert_eq!(Move::new_on_board("d7c8r", &board).get_kind(), MoveKind::RookCapturePromotion);
        assert_eq!(Move::new_on_board("d7c8q", &board).get_kind(), MoveKind::QueenCapturePromotion);
    }
}