use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece(PieceType, Color);

#[derive(Clone)]
pub struct Game {
    pub turn: Color,
    pub pieces: HashMap<(usize, usize), Piece>,
}

#[derive(Debug)]
pub struct Command {
    pub special: Option<Special>,
    pub piece: PieceType,
    pub from: Option<(Option<usize>, Option<usize>)>,
    pub to: (usize, usize),
    pub takes: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Special {
    Castle,
    LongCastle,
    Check,
    Checkmate,
    // EnPassant,
    // Promotion,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChessError {
    InvalidMove,
}

impl std::error::Error for ChessError {}

impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChessError::InvalidMove => write!(f, "Invalid move"),
        }
    }
}

use regex::Regex;
use lazy_static::lazy_static;

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        if input.len() < 2 {
            return None;
        }
        println!("Parsing: {}", input);
        lazy_static! {
            static ref NOTATION_PATTERN: Regex = Regex::new(
                r"^(?P<piece>[NBRQK])?(?P<from_col>[a-h])?(?P<from_row>[1-8])?(?P<takes>x)?(?P<to>[a-h][1-8])(?P<promotion>=[NBRQK])?(?P<check>\+|#)?$|^(?P<castle>O-O|O-O-O)?$"
            ).unwrap();
        }
        let captures = NOTATION_PATTERN.captures(input)?;
        if input == "O-O" {
            return Some(Self {
                from: None,
                piece: PieceType::King,
                special: Some(Special::Castle),
                takes: false,
                to: (0, 0),
            });
        }
        if input == "O-O-O" {
            return Some(Self {
                from: None,
                piece: PieceType::King,
                special: Some(Special::LongCastle),
                takes: false,
                to: (0, 0),
            });
        }
        let piece = match captures.name("piece") {
            Some(piece) =>
                match piece.as_str() {
                    "N" => PieceType::Knight,
                    "B" => PieceType::Bishop,
                    "R" => PieceType::Rook,
                    "Q" => PieceType::Queen,
                    "K" => PieceType::King,
                    _ => {
                        return None;
                    }
                }
            None => PieceType::Pawn,
        };
        let from_col = match captures.name("from_col") {
            Some(from_col) =>
                Some(letter_to_column_index(from_col.as_str().chars().next().unwrap())),
            None => None,
        };
        let from_row = match captures.name("from_row") {
            Some(from_row) => Some(from_row.as_str().parse::<usize>().unwrap() - 1),
            None => None,
        };
        let takes = captures.name("takes").is_some();
        if from_row.is_some() || from_col.is_some() {
            if piece != PieceType::Knight && piece != PieceType::Rook && piece != PieceType::Pawn {
                return None;
            }
            if piece == PieceType::Pawn && !takes {
                return None;
            }
        }
        let to = captures.name("to").unwrap().as_str();
        let check = match captures.name("check") {
            Some(check) => {
                match check.as_str() {
                    "+" => Some(Special::Check),
                    "#" => Some(Special::Checkmate),
                    _ => None,
                }
            }
            _ => None,
        };
        return Some(Self {
            from: if from_col.is_some() || from_row.is_some() {
                Some((from_col, from_row))
            } else {
                None
            },
            piece,
            special: check,
            takes,
            to: notation_to_coords(to).unwrap(),
        });
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Color::White,
            pieces: [
                ((0, 0), Piece(PieceType::Rook, Color::White)),
                ((1, 0), Piece(PieceType::Knight, Color::White)),
                ((2, 0), Piece(PieceType::Bishop, Color::White)),
                ((3, 0), Piece(PieceType::Queen, Color::White)),
                ((4, 0), Piece(PieceType::King, Color::White)),
                ((5, 0), Piece(PieceType::Bishop, Color::White)),
                ((6, 0), Piece(PieceType::Knight, Color::White)),
                ((7, 0), Piece(PieceType::Rook, Color::White)),
                ((0, 1), Piece(PieceType::Pawn, Color::White)),
                ((1, 1), Piece(PieceType::Pawn, Color::White)),
                ((2, 1), Piece(PieceType::Pawn, Color::White)),
                ((3, 1), Piece(PieceType::Pawn, Color::White)),
                ((4, 1), Piece(PieceType::Pawn, Color::White)),
                ((5, 1), Piece(PieceType::Pawn, Color::White)),
                ((6, 1), Piece(PieceType::Pawn, Color::White)),
                ((7, 1), Piece(PieceType::Pawn, Color::White)),
                ((0, 7), Piece(PieceType::Rook, Color::Black)),
                ((1, 7), Piece(PieceType::Knight, Color::Black)),
                ((2, 7), Piece(PieceType::Bishop, Color::Black)),
                ((3, 7), Piece(PieceType::Queen, Color::Black)),
                ((4, 7), Piece(PieceType::King, Color::Black)),
                ((5, 7), Piece(PieceType::Bishop, Color::Black)),
                ((6, 7), Piece(PieceType::Knight, Color::Black)),
                ((7, 7), Piece(PieceType::Rook, Color::Black)),
                ((0, 6), Piece(PieceType::Pawn, Color::Black)),
                ((1, 6), Piece(PieceType::Pawn, Color::Black)),
                ((2, 6), Piece(PieceType::Pawn, Color::Black)),
                ((3, 6), Piece(PieceType::Pawn, Color::Black)),
                ((4, 6), Piece(PieceType::Pawn, Color::Black)),
                ((5, 6), Piece(PieceType::Pawn, Color::Black)),
                ((6, 6), Piece(PieceType::Pawn, Color::Black)),
                ((7, 6), Piece(PieceType::Pawn, Color::Black)),
            ]
                .iter()
                .cloned()
                .collect::<HashMap<(usize, usize), Piece>>(),
        }
    }

    pub fn next(&mut self, input: Command) -> Result<(), ChessError> {
        let Command { to, from, piece, takes, .. } = input;
        let Game { turn: color, .. } = self;
        match self.pieces.get(&to) {
            Some(_) => {
                if !takes {
                    return Err(ChessError::InvalidMove);
                }
            }
            None => {
                if takes {
                    return Err(ChessError::InvalidMove);
                }
            }
        }
        match input.piece {
            PieceType::Pawn => {
                let diff = |curr: usize, diff: usize| {
                    match color {
                        Color::White => curr - diff,
                        Color::Black => curr + diff,
                    }
                };
                // can be optimized
                let mut possible_coords = vec![];
                if takes {
                    let from_col = from.unwrap().0.unwrap();
                    if from_col.abs_diff(to.0) != 1 {
                        return Err(ChessError::InvalidMove);
                    }
                    possible_coords.push((from_col, diff(to.1, 1)));
                } else {
                    possible_coords.push((to.0, diff(to.1, 1)));
                    if
                        (*color == Color::White && to.1 == 3) ||
                        (*color == Color::Black && to.1 == 4)
                    {
                        possible_coords.push((to.0, diff(to.1, 2)));
                    }
                }
                let mut found = false;
                for coords in possible_coords {
                    match self.pieces.get(&coords) {
                        Some(ref _piece @ Piece(PieceType::Pawn, _color)) if _color == color => {
                            found = true;
                            let pawn = self.pieces.remove(&coords).unwrap();
                            self.pieces.insert(to, pawn);
                        }
                        _ => {}
                    }
                }
                if !found {
                    return Err(ChessError::InvalidMove);
                }
            }
            PieceType::Knight => {
                let possible_coords = [
                    (to.0.checked_add(1), to.1.checked_add(2)),
                    (to.0.checked_add(1), to.1.checked_sub(2)),
                    (to.0.checked_sub(1), to.1.checked_add(2)),
                    (to.0.checked_sub(1), to.1.checked_sub(2)),
                    (to.0.checked_add(2), to.1.checked_add(1)),
                    (to.0.checked_add(2), to.1.checked_sub(1)),
                    (to.0.checked_sub(2), to.1.checked_add(1)),
                    (to.0.checked_sub(2), to.1.checked_sub(1)),
                ]
                    .iter()
                    .filter_map(|(x, y)| {
                        match (x, y) {
                            (Some(x), Some(y)) if *x < 8 && *y < 8 => Some((*x, *y)),
                            _ => None,
                        }
                    })
                    .collect::<Vec<(usize, usize)>>();
                let mut found = false;
                for coords in possible_coords {
                    match self.pieces.get(&coords) {
                        Some(ref _piece @ Piece(PieceType::Knight, _color)) if _color == color => {
                            found = true;
                            let knight = self.pieces.remove(&coords).unwrap();
                            self.pieces.insert(to, knight);
                            break;
                        }
                        _ => {}
                    }
                }
                if !found {
                    return Err(ChessError::InvalidMove);
                }
            }
            PieceType::Rook => {}
            PieceType::Bishop => {}
            PieceType::King => {}
            PieceType::Queen => {}
        }
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        Ok(())
    }
}

fn notation_to_coords(notation: &str) -> Option<(usize, usize)> {
    let mut chars = notation.chars();
    let x = (chars.next().unwrap() as usize) - ('a' as usize);
    let y = (chars.next().unwrap() as usize) - ('1' as usize);
    if x > 7 || y > 7 {
        return None;
    }
    Some((x, y))
}

fn letter_to_column_index(letter: char) -> usize {
    let letter = letter.to_ascii_lowercase();
    if letter < 'a' || letter > 'h' {
        panic!("How did we get here? I thought we checked this already.");
    }
    (letter as usize) - ('a' as usize)
}

fn coords_to_notation(coords: (usize, usize)) -> String {
    let x = (coords.0 as u8) + ('a' as u8);
    let y = (coords.1 as u8) + ('1' as u8);
    format!("{}{}", x as char, y as char)
}