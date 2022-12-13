use std::io;
#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
struct Piece {
    piece: PieceType,
    coords: (usize, usize),
}

impl Piece {
    fn from(piece: PieceType, coords: (usize, usize)) -> Self {
        Self { piece, coords }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

struct Game {
    turn: Color,
    pieces: [Vec<Piece>; 2],
}

struct Command {
    special: Option<Special>,
    piece: PieceType,
    to: (usize, usize),
    takes: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Special {
    Castle,
    LongCastle,
    Check,
    Checkmate,
    // EnPassant,
    // Promotion,
}

impl Command {
    fn parse(input: &str) -> Self {
        if input == "O-O" {
            return Self {
                piece: PieceType::King,
                to: (6, 0),
                takes: false,
                special: Some(Special::Castle),
            };
        }
        if input == "O-O-O" {
            return Self {
                piece: PieceType::King,
                to: (2, 0),
                takes: false,
                special: Some(Special::LongCastle),
            };
        }
        let mut chars = input.chars();
        let piece;
        match chars.next().unwrap() {
            'K' => {
                piece = PieceType::King;
            }
            'Q' => {
                piece = PieceType::Queen;
            }
            'R' => {
                piece = PieceType::Rook;
            }
            'B' => {
                piece = PieceType::Bishop;
            }
            'N' => {
                piece = PieceType::Knight;
            }
            'a'..='h' => {
                piece = PieceType::Pawn;
                let (to, last_char, takes);
                match chars.next().unwrap() {
                    '1'..='8' => {
                        to = notation_to_coords(&input[0..=1]);
                        last_char = chars.next();
                        takes = false;
                    }
                    'x' => {
                        to = notation_to_coords(&input[2..=3]);
                        last_char = chars.nth(2);
                        takes = true;
                    }
                    _ => {
                        panic!();
                    }
                }
                let special = if let Some(str) = last_char {
                    match str {
                        '+' => { Some(Special::Check) }
                        '#' => { Some(Special::Checkmate) }
                        _ => { panic!() }
                    }
                } else {
                    None
                };
                return Self {
                    piece,
                    to,
                    takes,
                    special,
                };
            }
            _ => {
                panic!("Invalid piece");
            }
        }
        let (to, takes, last_char);
        match chars.next().unwrap() {
            'a'..='h' => {
                to = notation_to_coords(&input[1..=2]);
                takes = false;
                last_char = chars.nth(1);
            }
            'x' => {
                to = notation_to_coords(&input[2..=3]);
                takes = true;
                last_char = chars.nth(2);
            }
            _ => {
                panic!("Invalid piece");
            }
        }
        let special = if let Some(str) = last_char {
            match str {
                '+' => { Some(Special::Check) }
                '#' => { Some(Special::Checkmate) }
                _ => { panic!() }
            }
        } else {
            None
        };
        return Self {
            to,
            takes,
            piece,
            special,
        };
    }
}

impl Game {
    fn new() -> Game {
        Game {
            turn: Color::White,
            pieces: [
                vec![
                    Piece::from(PieceType::Pawn, (0, 1)),
                    Piece::from(PieceType::Pawn, (1, 1)),
                    Piece::from(PieceType::Pawn, (2, 1)),
                    Piece::from(PieceType::Pawn, (3, 1)),
                    Piece::from(PieceType::Pawn, (4, 1)),
                    Piece::from(PieceType::Pawn, (5, 1)),
                    Piece::from(PieceType::Pawn, (6, 1)),
                    Piece::from(PieceType::Pawn, (7, 1)),
                    Piece::from(PieceType::Rook, (0, 0)),
                    Piece::from(PieceType::Rook, (7, 0)),
                    Piece::from(PieceType::Knight, (1, 0)),
                    Piece::from(PieceType::Knight, (6, 0)),
                    Piece::from(PieceType::Bishop, (2, 0)),
                    Piece::from(PieceType::Bishop, (5, 0)),
                    Piece::from(PieceType::Queen, (3, 0)),
                    Piece::from(PieceType::King, (4, 0))
                ],
                vec![
                    Piece::from(PieceType::Pawn, (0, 6)),
                    Piece::from(PieceType::Pawn, (1, 6)),
                    Piece::from(PieceType::Pawn, (2, 6)),
                    Piece::from(PieceType::Pawn, (3, 6)),
                    Piece::from(PieceType::Pawn, (4, 6)),
                    Piece::from(PieceType::Pawn, (5, 6)),
                    Piece::from(PieceType::Pawn, (6, 6)),
                    Piece::from(PieceType::Pawn, (7, 6)),
                    Piece::from(PieceType::Rook, (0, 7)),
                    Piece::from(PieceType::Rook, (7, 7)),
                    Piece::from(PieceType::Knight, (1, 7)),
                    Piece::from(PieceType::Knight, (6, 7)),
                    Piece::from(PieceType::Bishop, (2, 7)),
                    Piece::from(PieceType::Bishop, (5, 7)),
                    Piece::from(PieceType::Queen, (3, 7)),
                    Piece::from(PieceType::King, (4, 7))
                ],
            ],
        }
    }

    fn next(&mut self, input: Command) {
        let Command { to, piece, takes, .. } = input;
        let index = if self.turn == Color::White { 0 } else { 1 };
        let pieces = &self.pieces[index];
        self.pieces[index] = pieces
            .iter()
            .map(|piece| { *piece })
            .collect();
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        println!("What is your move?");
        stdin.read_line(&mut input).unwrap();
        println!("{:?}", input);
        input.clear();
    }
}

fn notation_to_coords(notation: &str) -> (usize, usize) {
    let mut chars = notation.chars();
    let x = (chars.next().unwrap() as usize) - ('a' as usize);
    let y = (chars.next().unwrap() as usize) - ('1' as usize);
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let command = Command::parse("Kd4");
        assert_eq!(command.piece, PieceType::King);
        assert_eq!(command.to, (3, 3));
        assert_eq!(command.takes, false);

        let command = Command::parse("Qd4");
        assert_eq!(command.piece, PieceType::Queen);
        assert_eq!(command.to, (3, 3));
        assert_eq!(command.takes, false);

        let command = Command::parse("Rxa8");
        assert_eq!(command.piece, PieceType::Rook);
        assert_eq!(command.to, (0, 7));
        assert_eq!(command.takes, true);

        let command = Command::parse("a4");
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (0, 3));
        assert_eq!(command.takes, false);

        let command = Command::parse("axd4");
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (3, 3));
        assert_eq!(command.takes, true);

        // write me some more tests for more moves like O-O-O, Bxh8, axd3, etc.
        let command = Command::parse("Bxh8");
        assert_eq!(command.piece, PieceType::Bishop);
        assert_eq!(command.to, (7, 7));
        assert_eq!(command.takes, true);

        let command = Command::parse("axd3");
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (3, 2));
        assert_eq!(command.takes, true);

        let command = Command::parse("O-O-O");
        assert_eq!(command.piece, PieceType::King);
        assert_eq!(command.special, Some(Special::LongCastle));

        let command = Command::parse("d4+");
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (3, 3));
        assert_eq!(command.takes, false);
        assert_eq!(command.special, Some(Special::Check));

        let command = Command::parse("Ba3#");
        assert_eq!(command.piece, PieceType::Bishop);
        assert_eq!(command.to, (0, 2));
        assert_eq!(command.takes, false);
        assert_eq!(command.special, Some(Special::Checkmate));

        let command = Command::parse("Qxh8+");
        assert_eq!(command.piece, PieceType::Queen);
        assert_eq!(command.to, (7, 7));
        assert_eq!(command.takes, true);
        assert_eq!(command.special, Some(Special::Check));
    }
}