use crate::engine::board::Case;
use crate::engine::moves::SimpleKind::{Quiet, DoublePawnPush, KingCastle, QueenCastle};
use crate::engine::moves::CaptureKind::{Simple, EnPassant};
use crate::engine::moves::PromotionKind::{Knight, Bishop, Rook, Queen};
use crate::engine::moves::MoveKind::{SimpleCapture, KnightCapturePromotion, BishopCapturePromotion, RookCapturePromotion, QueenCapturePromotion, KnightPromotion, BishopPromotion, RookPromotion, QueenPromotion, EnPassantCapture};

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
                match self.kind.promotion{
                    Knight => KnightCapturePromotion,
                    Bishop => BishopCapturePromotion,
                    Rook => RookCapturePromotion,
                    Queen => QueenCapturePromotion
                }
            }
        }
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
}
