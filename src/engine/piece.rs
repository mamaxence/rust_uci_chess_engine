use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use crate::engine::piece::PieceKind::{King, Queen, Rook, Bishop, Knight, Pawn};
use crate::engine::piece::Color::{White, Black};

#[derive(Debug, Copy, Clone)]
pub struct PieceParseError;

#[derive(Debug, Copy, Clone)]
pub enum Color{
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub enum PieceKind{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl fmt::Display for PieceKind{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            King => {write!(f, "k")}
            Queen => {write!(f, "q")}
            Rook => {write!(f, "r")}
            Bishop => {write!(f, "b")}
            Knight => {write!(f, "n")}
            Pawn => {write!(f, "p")}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Piece{
    kind: PieceKind,
    color: Color,
}

impl fmt::Display for Piece{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.color {
            White => {write!(f, "{}", self.kind.to_string().to_uppercase())}
            Black => {write!(f, "{}", self.kind.to_string())}
        }
    }
}

impl FromStr for Piece{
    type Err = PieceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "K" => Ok(Piece{kind:King, color:White}),
            "Q" => Ok(Piece{kind:Queen, color:White}),
            "R" => Ok(Piece{kind:Rook, color:White}),
            "B" => Ok(Piece{kind:Bishop, color:White}),
            "N" => Ok(Piece{kind:Knight, color:White}),
            "P" => Ok(Piece{kind:Pawn, color:White}),
            "k" => Ok(Piece{kind:King, color:Black}),
            "q" => Ok(Piece{kind:Queen, color:Black}),
            "r" => Ok(Piece{kind:Rook, color:Black}),
            "b" => Ok(Piece{kind:Bishop, color:Black}),
            "n" => Ok(Piece{kind:Knight, color:Black}),
            "p" => Ok(Piece{kind:Pawn, color:Black}),
            _ => Err(PieceParseError)
        }
    }
}
