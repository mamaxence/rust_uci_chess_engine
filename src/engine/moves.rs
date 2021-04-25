use crate::engine::board::{Case, ParseCaseError};
use crate::engine::moves::SimpleKind::{Quiet, DoublePawnPush, KingCastle, QueenCastle};
use crate::engine::moves::CaptureKind::{Simple, EnPassant};
use crate::engine::moves::PromotionKind::{Knight, Bishop, Rook, Queen};
use crate::engine::moves::MoveKind::{SimpleCapture, KnightCapturePromotion, BishopCapturePromotion, RookCapturePromotion, QueenCapturePromotion, KnightPromotion, BishopPromotion, RookPromotion, QueenPromotion, EnPassantCapture};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
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
    pub fn new_move(from: Case, to: Case, kind: MoveKind) -> Self{
        Move{from, to, flags: kind.into()}
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
            match s.chars().collect::<Vec<char>>()[5] {
                'q' => Ok(Move::new_move(from, to, MoveKind::QueenPromotion)),
                'k' => Ok(Move::new_move(from, to, MoveKind::KnightPromotion)),
                'b' => Ok(Move::new_move(from, to, MoveKind::BishopPromotion)),
                'r' => Ok(Move::new_move(from, to, MoveKind::RookPromotion)),
                _ => Err(MoveParseError)
            }
        } else{
            Ok(Move::new_move(from, to, MoveKind::Quiet))
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::engine::moves::Move;
    use crate::engine::board::Case;
    use crate::engine::moves::MoveKind::DoublePawnPush;

    #[test]
    fn test_print_move() {
        let m1 = Move::new_move(Case::new_from_str("e2"), Case::new_from_str("e4"), DoublePawnPush);
        println!("{}", m1)
    }
}