#![feature(test)]
use crate::engine::board::{Board, Case, Dir};
use crate::engine::moves::{Move, MoveKind};
use crate::engine::piece::{Color, PieceKind, Piece};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs::{read, read_to_string};
use std::fmt::format;


impl Board{

    /// get all legal moves for the current side
    pub fn get_moves(&self) -> Vec<Move>{
        (0..64).map(|i| Case::new(i))
            .filter_map(|p| self.get_moves_for_case(&p))
            .flatten() // vector for psedo legal move
            .filter(|mv| self.is_move_legal(mv)) // filter mate
            .collect()
    }

    /// Check the legality of a move (wether or not the king will be in check)
    pub fn is_move_legal(&self, mv: &Move) -> bool{
        let new_board = self.apply_move(mv);
        let attacked_case: HashSet<Case> = HashSet::from_iter(new_board.get_attacked_case(&new_board.side).into_iter());

        // does king is in attacked cases
        !attacked_case.contains(&new_board.get_kind_pos(&new_board.side.flip()))
    }

    /// Return the list of case the given color currently attack
    fn get_attacked_case(&self, color: &Color) -> Vec<Case>{
        let mut new_board = self.clone();
        new_board.side = *color;
        (0..64).map(|i| Case::new(i))
            .filter_map(|p| new_board.get_attack_move_for_case(&p))
            .flatten()
            .map(|mv| mv.to).collect()
    }

    /// Get pseudo legal moves for the piece on case
    fn get_moves_for_case(&self, case: &Case) -> Option<Vec<Move>>{
        if let Some(Piece{kind, color}) = self[case]{
            if color != self.side{
                return None
            }
            let mut moves = match kind {
                PieceKind::King => self.castle_move(),
                PieceKind::Pawn => self.pawn_quiet_moves(case),
                _ => Vec::new()
            };
            moves.append(&mut self.get_attack_move_for_case(case).unwrap());
            Some(moves)
        }else{
            None
        }
    }

    /// Get potential attack for piece in the case (all but not pawn move or rock)
    fn get_attack_move_for_case(&self, case: &Case) -> Option<Vec<Move>>{
        if let Some(Piece{kind, color}) = self[case]{
            if color != self.side{
                return None
            }
            Some(match kind{
                PieceKind::Queen => self.moves_for_dir(case,
                                                       &[Dir::Up,
                                                           Dir::UpLeft,
                                                           Dir::UpRight,
                                                           Dir::Right,
                                                           Dir::Left,
                                                           Dir::DownLeft,
                                                           Dir::DownRight,
                                                           Dir::Down], 8),
                PieceKind::Rook => self.moves_for_dir(case,
                                                      &[Dir::Up,
                                                          Dir::Right,
                                                          Dir::Left,
                                                          Dir::Down], 8),
                PieceKind::Bishop => self.moves_for_dir(case,
                                                        &[Dir::UpLeft,
                                                            Dir::UpRight,
                                                            Dir::DownLeft,
                                                            Dir::DownRight], 8),
                PieceKind::Knight => self.moves_for_dir(case,
                                                        &[
                                                            Dir::Cav1,
                                                            Dir::Cav2,
                                                            Dir::Cav4,
                                                            Dir::Cav5,
                                                            Dir::Cav7,
                                                            Dir::Cav8,
                                                            Dir::Cav10,
                                                            Dir::Cav11
                                                        ], 1),
                PieceKind::King => self.moves_for_dir(case,
                                                      &[Dir::Up,
                                                          Dir::UpLeft,
                                                          Dir::UpRight,
                                                          Dir::Right,
                                                          Dir::Left,
                                                          Dir::DownLeft,
                                                          Dir::DownRight,
                                                          Dir::Down], 1),
                PieceKind::Pawn => self.pawn_attack_moves(case),
            })
        }else{
            None
        }
    }

    /// Get pseudo legal moves for a pawn situated in case
    fn pawn_quiet_moves(&self, case: &Case) -> Vec<Move>{
        let mut moves: Vec<Move> = Vec::new();
        let (front, start_line, promotion_line) = match self.side {
            Color::White => (Dir::Up, 1, 7),
            Color::Black => (Dir::Down, 6, 0),
        };

        // simple & double pawn push
        let front_case = &case.get_neighbour(front, 1).unwrap();
        if self[front_case].is_none(){
            if front_case.get_line() == promotion_line {
                moves.push(Move::new(case.clone(), front_case.clone(), MoveKind::QueenPromotion));
                moves.push(Move::new(case.clone(), front_case.clone(), MoveKind::BishopPromotion));
                moves.push(Move::new(case.clone(), front_case.clone(), MoveKind::RookPromotion));
                moves.push(Move::new(case.clone(), front_case.clone(), MoveKind::KnightPromotion));
            } else{
                moves.push(Move::new(case.clone(), front_case.clone(), MoveKind::Quiet));
            }

            if case.get_line() == start_line && self[&case.get_neighbour(front, 2).unwrap()].is_none(){
                moves.push(Move::new(case.clone(), case.get_neighbour(front, 2).unwrap(), MoveKind::DoublePawnPush));
            }
        }
        moves
    }

    /// Return the cases controlled by a pawn
    fn get_pawn_control_case(&self, case: &Case) -> Vec<Case>{
        let mut cases: Vec<Case> = Vec::new();
        let (front_left, front_right) = match self[case].unwrap().color {
            Color::White => (Dir::UpLeft, Dir::UpRight),
            Color::Black => (Dir::DownLeft, Dir::DownRight),
        };
        if let Some(case) = case.get_neighbour(front_right, 1){
            cases.push(case);
        }
        if let Some(case) = case.get_neighbour(front_left, 1){
            cases.push(case);
        }
        cases
    }

    fn pawn_attack_moves(&self, case: &Case) -> Vec<Move>{
        let mut moves: Vec<Move> = Vec::new();
        let (front_left, front_right,  promotion_line) = match self.side {
            Color::White => (Dir::UpLeft, Dir::UpRight, 7),
            Color::Black => (Dir::DownLeft, Dir::DownRight, 0),
        };

        for &side in [front_left, front_right].iter(){
            if let Some(target) = case.get_neighbour(side, 1){
                if self.is_adversary(&target, &self.side){
                    if target.get_line() == promotion_line {
                        moves.push(Move::new(case.clone(), target.clone(), MoveKind::QueenCapturePromotion));
                        moves.push(Move::new(case.clone(), target.clone(), MoveKind::BishopCapturePromotion));
                        moves.push(Move::new(case.clone(), target.clone(), MoveKind::RookCapturePromotion));
                        moves.push(Move::new(case.clone(), target.clone(), MoveKind::KnightCapturePromotion));
                    } else {
                        moves.push(Move::new(case.clone(), target, MoveKind::SimpleCapture))
                    }
                }
                if let Some(en_passant_target) = self.en_passant{
                    if target == en_passant_target{
                        moves.push(Move::new(case.clone(), target, MoveKind::EnPassantCapture))
                    }
                }
            }
        }
        moves
    }

    /// get castle move if possible
    fn castle_move(&self) -> Vec<Move>{
        let mut mv = Vec::new();

        let (king_case, queen_case, mut can_king, mut can_queen) = match self.side {
            Color::White => ([Case::new(4), Case::new(5), Case::new(6)],
                             [Case::new(4), Case::new(3), Case::new(2), Case::new(1)],
                             self.castle.white_king, self.castle.white_queen),
            Color::Black => ([Case::new(60), Case::new(61), Case::new(62)],
                             [Case::new(60), Case::new(59), Case::new(58), Case::new(57)],
                             self.castle.black_king, self.castle.black_queen),
        };

        if !can_king && ! can_queen{
            return mv
        }

        match (self[&king_case[1]], self[&king_case[2]]){
            (Some(p), _) => can_king = false,
            (_, Some(p)) => can_king = false,
            (_, _) => {}
        }
        match (self[&queen_case[1]], self[&queen_case[2]], self[&queen_case[3]]){
            (Some(p), _, _) => can_queen = false,
            (_, Some(p), _) => can_queen = false,
            (_, _, Some(p)) => can_queen = false,
            (_, _, _) => {}
        }

        if !can_king && ! can_queen{
            return mv
        }
        let color = self.side.flip();
        let attacked_cases = self.get_controled_cases(&color);
        if can_king && !attacked_cases.contains(&king_case[0])
            && !attacked_cases.contains(&king_case[1])
            && !attacked_cases.contains(&king_case[2]){
            mv.push(Move::new(king_case[0], king_case[2], MoveKind::KingCastle))
        };
        if can_queen && !attacked_cases.contains(&queen_case[0])
            && !attacked_cases.contains(&queen_case[1])
            && !attacked_cases.contains(&queen_case[2]){
            mv.push(Move::new(queen_case[0], queen_case[2], MoveKind::QueenCastle))
        };
        mv
    }

    /// Get controled cases by piece of given color
    /// Not the same than attacked_cases because a pawn can control a case without being able to
    /// move to it (empty cases)
    fn get_controled_cases(&self, color: &Color) -> HashSet<Case> {
        let mut attacked_cases: HashSet<Case> = HashSet::from_iter(self.get_attacked_case(color).into_iter());
        attacked_cases.extend(self.get_pawn_controled_cases(color).iter());
        attacked_cases
    }

    /// Return cases controled by pawn of given color
    fn get_pawn_controled_cases(&self, color: &Color) -> Vec<Case> {
        (0..64)
            .map(|i| Case::new(i))
            .filter(|c| {
                if let Some(Piece { kind: PieceKind::Pawn, color: col }) = self[c] { col == *color } else { false }
            }
            )
            .map(|c| self.get_pawn_control_case(&c))
            .flatten()
            .collect()
    }

    /// Get pseudo legal moves for piece given a list of dir and a max distance
    fn moves_for_dir(&self,case: &Case, dirs: &[Dir], max: usize) -> Vec<Move>{
        dirs.into_iter()
            .map(|dir| self.all_moves_in_dir(case, dir, max).into_iter())
            .flatten()
            .collect()
    }

    /// Create all move in dir until, same color, capture or end of board
    fn all_moves_in_dir(&self, from: &Case, dir: &Dir, max: usize) -> Vec<Move>{
        let mut res: Vec<Move> = Vec::new();
        let mut dist = 1;
        while let Some(to) = from.get_neighbour(*dir, dist){
            if let Some(Piece{color, kind}) = self[&to]{
                if {color != self.side}{
                    res.push(Move::new(from.clone(), to.clone(), MoveKind::SimpleCapture))
                }
                break
            } else{
                res.push(Move::new(from.clone(), to.clone(), MoveKind::Quiet))
            }
            if dist == max{
                break
            }
            dist += 1
        }
        res
    }

    /// Is there an adversary on case for color
    fn is_adversary(&self, case: &Case, color: &Color) -> bool{
        if let Some(piece) = self[case] {
            &piece.color != color
        } else{
            false
        }
    }

    /// Get the king position for color
    fn get_kind_pos(&self, color: &Color) -> Case{
        for i in 0..64{
            if let Some(Piece{color: col, kind: PieceKind::King}) = self[i]{
                if col == *color{
                    return Case::new(i)
                }
            }
        }
        panic!(format!("king not found on board \n{}", self.to_fen()))
    }
}
#[cfg(test)]
mod tests{
    use crate::engine::board::Board;
    use crate::engine::moves::Move;
    use test::Bencher;

    fn perft(board: &Board, depth: u64) -> u64{
        if depth == 0{
            return 1
        }
        let mut sum = 0;
        for mv in board.get_moves()
            .into_iter(){
            let nb = board.apply_move(&mv);
            let perft = sub_perft(&nb, depth-1);
            sum += perft;
            println!("{}:  {}", mv, perft)
        }
        sum
    }

    fn sub_perft(board: &Board, depth: u64) -> u64{
        if depth == 0{
            return 1
        }
        board.get_moves()
            .into_iter()
            .map(|m| board.apply_move(&m))
            .map(|b| sub_perft(&b, depth-1))
            .sum()
    }

    #[test]
    fn test_mov_gen_start_board(){
        let board = Board::new_board();
        assert_eq!(20, perft(&board, 1));
        assert_eq!(400, perft(&board, 2));
        assert_eq!(8_902, perft(&board, 3));
        assert_eq!(197_281 , perft(&board, 4));
        assert_eq!(4_865_609, perft(&board, 5));
    }

    #[test]
    fn test_mov_gen_2(){
        let mut board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0");
        //board = board.apply_move(&"a1c1".parse().unwrap());
        //board = board.apply_move(&"h3g2".parse().unwrap());
        //assert_eq!(44, perft(&board, 1)); // after h3g2

        assert_eq!(48, perft(&board, 1));
        assert_eq!(2039, perft(&board, 2));
        assert_eq!(97_862, perft(&board, 3));
        assert_eq!(4_085_603, perft(&board, 4));

        let mut board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0");
        board = board.apply_move(&"a1c1".parse().unwrap());
        //assert_eq!(1968, perft(&board, 2)) // after a1c1

        let mut board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0");
        //board = board.apply_move(&"a1c1".parse().unwrap());
        //board = board.apply_move(&"h3g2".parse().unwrap());
        //assert_eq!(44, perft(&board, 1)); // after h3g2
    }

    #[test]
    fn test_mov_gen_3(){
        let board = Board::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0");
        assert_eq!(14, perft(&board, 1));
        assert_eq!(191, perft(&board, 2));
        assert_eq!(2_812, perft(&board, 3));
        assert_eq!(43_238, perft(&board, 4));
        assert_eq!(674_624, perft(&board, 5));
    }

    #[bench]
    fn test_mov_gen_3_bench(b: &mut Bencher){
        let board = Board::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0");
        assert_eq!(14, perft(&board, 1));
        assert_eq!(191, perft(&board, 2));
        assert_eq!(2_812, perft(&board, 3));
        assert_eq!(43_238, perft(&board, 4));
        assert_eq!(674_624, perft(&board, 5));
        assert_eq!(11_030_083, perft(&board, 6));
    }

    #[test]
    fn test_mov_gen_4() {
        let board = Board::new_from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        assert_eq!(6, perft(&board, 1));
        assert_eq!(264, perft(&board, 2));
        assert_eq!(9467, perft(&board, 3));
        assert_eq!(422_333, perft(&board, 4));
        assert_eq!(15_833_292, perft(&board, 5));
    }

    #[test]
    fn test_mov_gen_4_mirror(){
        let board = Board::new_from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
        assert_eq!(6, perft(&board, 1));
        assert_eq!(264, perft(&board, 2));
        assert_eq!(9467, perft(&board, 3));
        assert_eq!(422_333, perft(&board, 4));
        assert_eq!(15_833_292, perft(&board, 5));
    }

    #[test]
    fn test_mov_gen_5(){
        let board = Board::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
        assert_eq!(44, perft(&board, 1));
        assert_eq!(1486, perft(&board, 2));
        assert_eq!(62_379 , perft(&board, 3));
        assert_eq!(2_103_487 , perft(&board, 4));
    }

    #[test]
    fn test_mov_gen_6(){
        let board = Board::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
        let moves = board.get_moves();
        for mv in moves.iter(){
            println!("{}", mv)
        }
        assert_eq!(46, perft(&board, 1));
        assert_eq!(2_079, perft(&board, 2));
        assert_eq!(89_890, perft(&board, 3));
        assert_eq!(3_894_594, perft(&board, 4));
    }

    #[test]
    fn castle_under_attack(){
        // non regression test for castling under attack
        let board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q2/PPPBBPpP/1R2K2R w Kkq - 0 2");
        assert_eq!(44, perft(&board, 1))
    }
}
