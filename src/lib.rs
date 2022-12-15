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
pub enum GameState {
    InProgress,
    Checkmate(Color),
    Check(Color),
    Stalemate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece(PieceType, Color);

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self(piece_type, color)
    }
}

#[derive(Clone)]
pub struct Game {
    pub turn: Color,
    pub pieces: HashMap<(usize, usize), Piece>,
    pub state: GameState,
}

#[derive(Debug)]
pub struct Command {
    pub special: Option<Special>,
    pub piece: PieceType,
    pub from: (Option<usize>, Option<usize>),
    pub to: (usize, usize),
    pub takes: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Special {
    Castle,
    LongCastle,
    Check,
    Checkmate,
    EnPassant,
    Promotion,
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

use lazy_static::lazy_static;
use regex::Regex;

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        if input.len() < 2 {
            return None;
        }
        lazy_static! {
            static ref NOTATION_PATTERN: Regex = Regex::new(
                r"^(?P<piece>[NBRQK])?(?P<from_col>[a-h])?(?P<from_row>[1-8])?(?P<takes>x)?(?P<to>[a-h][1-8])(?P<promotion>=[NBRQK])?(?P<check>\+|#)?$|^(?P<castle>O-O|O-O-O)?$"
            ).unwrap();
        }
        let captures = NOTATION_PATTERN.captures(input)?;
        if input == "O-O" {
            return Some(Self {
                from: (None, None),
                piece: PieceType::King,
                special: Some(Special::Castle),
                takes: false,
                to: (0, 0),
            });
        }
        if input == "O-O-O" {
            return Some(Self {
                from: (None, None),
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
        let from_col = if let Some(from_col) = captures.name("from_col") {
            Some(letter_to_column_index(from_col.as_str().chars().next().unwrap()))
        } else {
            None
        };
        let from_row = if let Some(from_row) = captures.name("from_row") {
            Some(from_row.as_str().parse::<usize>().unwrap() - 1)
        } else {
            None
        };
        let takes = captures.name("takes").is_some();
        if from_row.is_some() || from_col.is_some() {
            if
                (piece != PieceType::Knight &&
                    piece != PieceType::Rook &&
                    piece != PieceType::Pawn) ||
                (!takes && piece == PieceType::Pawn)
            {
                return None;
            }
        }
        let to = captures.name("to").unwrap().as_str();
        let check = if let Some(check) = captures.name("check") {
            match check.as_str() {
                "+" => Some(Special::Check),
                "#" => Some(Special::Checkmate),
                _ => None,
            }
        } else {
            None
        };

        return Some(Self {
            from: (from_col, from_row),
            piece,
            takes,
            to: notation_to_coords(to).unwrap(),
            special: check,
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
            state: GameState::InProgress,
        }
    }

    pub fn play(&mut self, input: Command) -> Result<(), ChessError> {
        let Command { to, from, piece, takes, special } = input;
        let Game { turn: color, .. } = self;
        match special {
            Some(castle @ Special::LongCastle | castle @ Special::Castle) => {
                if piece != PieceType::King {
                    return Err(ChessError::InvalidMove);
                }
                let rook_col = match castle {
                    Special::LongCastle => 0,
                    Special::Castle => 7,
                    _ => unreachable!(),
                };
                let home_row = match color {
                    Color::White => 0,
                    Color::Black => 7,
                };
                let (from_king, from_rook) = ((4, home_row), (rook_col, home_row));
                if self.pieces.get(&from_king).is_none() || self.pieces.get(&from_rook).is_none() {
                    return Err(ChessError::InvalidMove);
                }
                let (to_king, to_rook) = match castle {
                    Special::LongCastle => ((2, home_row), (3, home_row)),
                    Special::Castle => ((6, home_row), (5, home_row)),
                    _ => unreachable!(),
                };
                if self.pieces.get(&to_king).is_some() || self.pieces.get(&to_rook).is_some() {
                    return Err(ChessError::InvalidMove);
                }
                let range = match castle {
                    Special::LongCastle => 1..4,
                    Special::Castle => 5..7,
                    _ => unreachable!(),
                };
                for col in range {
                    if self.pieces.get(&(col, home_row)).is_some() {
                        return Err(ChessError::InvalidMove);
                    }
                }
                let king = self.pieces.remove(&from_king).unwrap();
                let rook = self.pieces.remove(&from_rook).unwrap();
                self.pieces.insert(to_king, king);
                self.pieces.insert(to_rook, rook);
                self.next_turn();
                return Ok(());
            }
            _ => {}
        }
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
        let mut directions: Option<Vec<(isize, isize)>> = None;
        let mut possible_coords: Option<Vec<(usize, usize)>> = None;
        match input.piece {
            PieceType::Pawn => {
                // todo refactor i dont like this
                let diff = |curr: usize, diff: usize| {
                    match color {
                        Color::White => curr - diff,
                        Color::Black => curr + diff,
                    }
                };
                let mut coords = vec![];
                if takes {
                    let from_col = from.0.unwrap();
                    if from_col.abs_diff(to.0) != 1 {
                        return Err(ChessError::InvalidMove);
                    }
                    coords.push((from_col, diff(to.1, 1)));
                } else {
                    coords.push((to.0, diff(to.1, 1)));
                    if
                        (*color == Color::White && to.1 == 3) ||
                        (*color == Color::Black && to.1 == 4)
                    {
                        coords.push((to.0, diff(to.1, 2)));
                    }
                }
                possible_coords = Some(coords);
            }
            PieceType::Knight => {
                possible_coords = Some(
                    [
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
                        .collect()
                );
            }
            PieceType::Rook => {
                directions = Some(vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);
            }
            PieceType::Bishop => {
                directions = Some(vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]);
            }
            PieceType::King => {
                possible_coords = Some(
                    [
                        (to.0.checked_add(1), to.1.checked_add(1)),
                        (to.0.checked_add(1), to.1.checked_sub(1)),
                        (to.0.checked_sub(1), to.1.checked_add(1)),
                        (to.0.checked_sub(1), to.1.checked_sub(1)),
                        (to.0.checked_add(1), Some(to.1)),
                        (to.0.checked_sub(1), Some(to.1)),
                        (Some(to.0), to.1.checked_add(1)),
                        (Some(to.0), to.1.checked_sub(1)),
                    ]
                        .iter()
                        .filter_map(|(x, y)| {
                            match (x, y) {
                                (Some(x), Some(y)) if *x < 8 && *y < 8 => Some((*x, *y)),
                                _ => None,
                            }
                        })
                        .collect()
                );
            }
            PieceType::Queen => {
                directions = Some(
                    vec![(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1)]
                );
            }
        }
        let mut piece_found = false;
        if let Some(directions) = directions {
            'all_directions: for direction in directions {
                let mut i = 0;
                'inner: loop {
                    let coords = match next_coords(to, direction, i) {
                        Some(coords) => coords,
                        None => {
                            break 'inner;
                        }
                    };
                    match self.pieces.get(&coords) {
                        Some(ref _piece @ Piece(_piecetype, _color)) if
                            _piecetype == &piece &&
                            _color == color &&
                            coords_match_from(coords, from)
                        => {
                            piece_found = true;
                            let moved_piece = self.pieces.remove(&coords).unwrap();
                            self.pieces.insert(to, moved_piece);
                            break 'all_directions;
                        }
                        Some(_) => {
                            break 'inner;
                        }
                        None => {}
                    }
                    i += 1;
                }
            }
        } else if let Some(possible_coords) = possible_coords {
            for coords in possible_coords {
                match self.pieces.get(&coords) {
                    Some(ref _piece @ Piece(_piecetype, _color)) if
                        _color == color &&
                        _piecetype == &piece &&
                        coords_match_from(coords, from)
                    => {
                        piece_found = true;
                        let moved_piece = self.pieces.remove(&coords).unwrap();
                        self.pieces.insert(to, moved_piece);
                        break;
                    }
                    _ => {}
                }
            }
        } else {
            return Err(ChessError::InvalidMove);
        }
        if !piece_found {
            return Err(ChessError::InvalidMove);
        }

        self.next_turn();
        Ok(())
    }

    fn next_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
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

fn coords_match_from(coords: (usize, usize), from: (Option<usize>, Option<usize>)) -> bool {
    match from {
        (Some(x), None) => coords.0 == x,
        (None, Some(y)) => coords.1 == y,
        (Some(x), Some(y)) => coords.0 == x && coords.1 == y,
        _ => true,
    }
}

fn next_coords(
    origin: (usize, usize),
    direction: (isize, isize),
    step: isize
) -> Option<(usize, usize)> {
    let x =
        (origin.0 as isize) +
        (direction.0 +
            (
                (if direction.0 < 0 {
                    -1 * step
                } else if direction.0 > 0 {
                    step
                } else {
                    0
                }) as isize
            ));
    let y =
        (origin.1 as isize) +
        (direction.1 +
            (
                (if direction.1 < 0 {
                    -1 * step
                } else if direction.1 > 0 {
                    step
                } else {
                    0
                }) as isize
            ));

    let (x, y) = (x.try_into(), y.try_into());
    match (x, y) {
        (Ok(x @ 0..=7), Ok(y @ 0..=7)) => Some((x, y)),
        _ => None,
    }
}