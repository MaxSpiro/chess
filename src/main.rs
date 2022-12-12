use std::io;
#[derive(Copy, Clone, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}

const STARTING_BOARD: [[Tile; 8]; 8] = [
    [
        Tile(Some(Piece::Rook(Color::White))),
        Tile(Some(Piece::Knight(Color::White))),
        Tile(Some(Piece::Bishop(Color::White))),
        Tile(Some(Piece::Queen(Color::White))),
        Tile(Some(Piece::King(Color::White))),
        Tile(Some(Piece::Bishop(Color::White))),
        Tile(Some(Piece::Knight(Color::White))),
        Tile(Some(Piece::Rook(Color::White))),
    ],
    [Tile(Some(Piece::Pawn(Color::White))); 8],
    [Tile(None); 8],
    [Tile(None); 8],
    [Tile(None); 8],
    [Tile(None); 8],
    [Tile(Some(Piece::Pawn(Color::Black))); 8],
    [
        Tile(Some(Piece::Rook(Color::Black))),
        Tile(Some(Piece::Knight(Color::Black))),
        Tile(Some(Piece::Bishop(Color::Black))),
        Tile(Some(Piece::Queen(Color::Black))),
        Tile(Some(Piece::King(Color::Black))),
        Tile(Some(Piece::Bishop(Color::Black))),
        Tile(Some(Piece::Knight(Color::Black))),
        Tile(Some(Piece::Rook(Color::Black))),
    ],
];

#[derive(Copy, Clone, Debug)]
struct Tile(Option<Piece>);

#[derive(Debug)]
struct Board {
    turn: Color,
    board: [[Tile; 8]; 8],
}

impl Board {
    fn new() -> Board {
        Board {
            turn: Color::White,
            board: STARTING_BOARD,
        }
    }

    fn next(&mut self, input: &str) {
        let mut chars = input.chars();
    }
}

fn main() {
    let board = Board::new();
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        println!("It is {:?}'s turn", board.turn);
        println!("What is your move?");
        stdin.read_line(&mut input).unwrap();
        println!("{:?}", input);
        input.clear();
    }
}