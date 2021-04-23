
fn main() {
    println!("Hello, world!");
}

mod board{
    use crate::board::PieceKind::{Rooks, Knights, Bishops, Queen, King, Pawns};
    use crate::board::Color::{White, Black};
    use std::ops::{Index, IndexMut};
    use std::fmt;
    use std::fmt::Formatter;
    use ansi_term::Color::{Blue,Green};
    use crate::main;

    #[derive(Debug, Copy, Clone)]
    enum Color{
        White,
        Black,
    }

    #[derive(Debug, Copy, Clone)]
    enum PieceKind{
        King,
        Queen,
        Rooks,
        Bishops,
        Knights,
        Pawns,
    }
    impl fmt::Display for PieceKind{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                King => {write!(f, "X")}
                Queen => {write!(f, "Q")}
                Rooks => {write!(f, "R")}
                Bishops => {write!(f, "B")}
                Knights => {write!(f, "K")}
                Pawns => {write!(f, "P")}
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Piece{
        kind: PieceKind,
        color: Color,
    }
    impl fmt::Display for Piece{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self.color {
                White => {write!(f, "{}", Green.paint(self.kind.to_string()).to_string())}
                Black => {write!(f, "{}", Blue.paint(self.kind.to_string()).to_string())}
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Board{
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
                        None => str.push_str(".")
                    }
                }
                str.push_str("\n");
            }
            str.push_str("  abcdefg");
            write!(f, "{}", str.to_string())
        }
    }
    impl Board{
        /// Create a new board with no pieces.
        fn new_empty_board() -> Self{
            Board{board: [None; 64]}
        }

        /// Create a new  board with starting position.
        fn new_board() -> Self{
            let mut board = Board::new_empty_board();
            board[0] = Some(Piece{kind:Rooks, color:White});
            board[1] = Some(Piece{kind:Knights, color:White});
            board[2] = Some(Piece{kind:Bishops, color:White});
            board[3] = Some(Piece{kind:Queen, color:White});
            board[4] = Some(Piece{kind:King, color:White});
            board[5] = Some(Piece{kind:Bishops, color:White});
            board[6] = Some(Piece{kind:Knights, color:White});
            board[7] = Some(Piece{kind:Rooks, color:White});
            for i in 8..16 {
                board[i] = Some(Piece{kind:Pawns, color:White});
            }
            for i in 48..56 {
                board[i] = Some(Piece{kind:Pawns, color: Black });
            }
            board[56] = Some(Piece{kind:Rooks, color: Black });
            board[57] = Some(Piece{kind:Knights, color: Black });
            board[58] = Some(Piece{kind:Bishops, color: Black });
            board[59] = Some(Piece{kind:Queen, color: Black });
            board[60] = Some(Piece{kind:King, color: Black });
            board[61] = Some(Piece{kind:Bishops, color: Black });
            board[62] = Some(Piece{kind:Knights, color: Black });
            board[63] = Some(Piece{kind:Rooks, color: Black });
            board
        }
    }

    #[cfg(test)]
    mod tests{
        use crate::board::{Board};

        #[test]
        fn debug_board(){
            let empty_board = Board::new_empty_board();
            println!("{:?}", empty_board);
            println!("{}", empty_board);
            let board = Board::new_board();
            println!("{:?}", board);
            println!("{}", board);
        }
    }
}
