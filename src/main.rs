use std::io;
mod engine;

fn main() {
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        uci_parser::parse_line(&input);
    }
}

mod uci_parser {

    pub fn parse_line(line: &str){
        let split_line: Vec<&str> = line[..(line.len()-1)].split(' ').collect();
        if split_line.is_empty() {
            eprintln!("Empty input!")
        }
        eprintln!("got: {:?}", split_line);
        match split_line[0] {
            "uci" => parse_uci(),
            "isready" => parse_isready(),
            "ucinewgame" => parse_ucinewgame(),
            "position" => parse_position(&split_line[1..]),
            _ => eprintln!("Unsuported opperation : {}", line)
        }
    }

    pub fn parse_uci(){
        println!("uciok"); // acknowledge the uci mode
    }

    pub fn parse_isready(){
        println!("readyok"); // acknowledge the engine is ready
    }

    pub fn parse_ucinewgame(){
        ()
    }

    use crate::engine::board::Board;

    pub fn parse_position(details: &[&str]){
        let board:Board = match details[0]{
            "startpos"=> Board::new_board(),
            fen=> Board::new_from_fen(fen),
        };
        eprintln!("created board:\n{}", board)
    }

}