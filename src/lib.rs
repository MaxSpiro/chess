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

pub struct Game {
    pub turn: Color,
    pub pieces: HashMap<(usize, usize), Piece>,
}

#[derive(Debug)]
pub struct Command {
    pub special: Option<Special>,
    pub piece: PieceType,
    pub to: (usize, usize),
    pub takes: bool,
    pub original: String,
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

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        if input.len() < 2 {
            return None;
        }
        let original = input.to_string();
        if input == "O-O" {
            return Some(Self {
                piece: PieceType::King,
                to: (6, 0),
                takes: false,
                special: Some(Special::Castle),
                original,
            });
        }
        if input == "O-O-O" {
            return Some(Self {
                piece: PieceType::King,
                to: (2, 0),
                takes: false,
                special: Some(Special::LongCastle),
                original,
            });
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
                        if let Some(coords) = notation_to_coords(&input[0..=1]) {
                            to = coords;
                        } else {
                            return None;
                        }
                        last_char = chars.next();
                        takes = false;
                    }
                    'x' => {
                        if let Some(coords) = notation_to_coords(&input[2..=3]) {
                            to = coords;
                        } else {
                            return None;
                        }
                        last_char = chars.nth(2);
                        takes = true;
                    }
                    c => {
                        return None;
                    }
                }
                let special = if let Some(str) = last_char {
                    match str {
                        '+' => { Some(Special::Check) }
                        '#' => { Some(Special::Checkmate) }
                        _ => {
                            return None;
                        }
                    }
                } else {
                    None
                };
                if special.is_some() && chars.next().is_some() {
                    return None;
                }
                return Some(Self {
                    piece,
                    to,
                    takes,
                    special,
                    original,
                });
            }
            _ => {
                return None;
            }
        }
        let (to, takes, last_char);
        match chars.next().unwrap() {
            'a'..='h' => {
                if let Some(coords) = notation_to_coords(&input[1..=2]) {
                    to = coords;
                } else {
                    return None;
                }
                takes = false;
                last_char = chars.nth(1);
            }
            'x' => {
                if let Some(coords) = notation_to_coords(&input[2..=3]) {
                    to = coords;
                } else {
                    return None;
                }
                takes = true;
                last_char = chars.nth(2);
            }
            _ => {
                return None;
            }
        }
        let special = if let Some(str) = last_char {
            match str {
                '+' => { Some(Special::Check) }
                '#' => { Some(Special::Checkmate) }
                _ => {
                    return None;
                }
            }
        } else {
            None
        };
        if special.is_some() && chars.next().is_some() {
            return None;
        }
        return Some(Self {
            to,
            takes,
            piece,
            special,
            original,
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
        let Command { to, piece, takes, original, .. } = input;
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
                    let column_index = letter_to_column_index(original.chars().next().unwrap());
                    if (column_index as i32).abs_diff(to.0 as i32) != 1 {
                        return Err(ChessError::InvalidMove);
                    }
                    possible_coords.push((
                        letter_to_column_index(original.chars().next().unwrap()),
                        diff(to.1, 1),
                    ));
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
                let mut possible_coords = vec![];
                // can be optimized
                for x in 0..8 {
                    for y in 0..8 {
                        if
                            ((x as i32) - (to.0 as i32)).abs() +
                                ((y as i32) - (to.1 as i32)).abs() == 3
                        {
                            possible_coords.push((x, y));
                        }
                    }
                }
                let mut found = false;
                for coords in possible_coords {
                    match self.pieces.get(&coords) {
                        Some(ref _piece @ Piece(PieceType::Knight, _color)) if _color == color => {
                            found = true;
                            let knight = self.pieces.remove(&coords).unwrap();
                            self.pieces.insert(to, knight);
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