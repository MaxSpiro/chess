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

#[derive(Clone, Debug)]
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
                ((1, 1), Piece(PieceType::Rook, Color::White)),
                ((2, 1), Piece(PieceType::Knight, Color::White)),
                ((3, 1), Piece(PieceType::Bishop, Color::White)),
                ((4, 1), Piece(PieceType::Queen, Color::White)),
                ((5, 1), Piece(PieceType::King, Color::White)),
                ((6, 1), Piece(PieceType::Bishop, Color::White)),
                ((7, 1), Piece(PieceType::Knight, Color::White)),
                ((8, 1), Piece(PieceType::Rook, Color::White)),
                ((1, 2), Piece(PieceType::Pawn, Color::White)),
                ((2, 2), Piece(PieceType::Pawn, Color::White)),
                ((3, 2), Piece(PieceType::Pawn, Color::White)),
                ((4, 2), Piece(PieceType::Pawn, Color::White)),
                ((5, 2), Piece(PieceType::Pawn, Color::White)),
                ((6, 2), Piece(PieceType::Pawn, Color::White)),
                ((7, 2), Piece(PieceType::Pawn, Color::White)),
                ((8, 2), Piece(PieceType::Pawn, Color::White)),
                ((1, 8), Piece(PieceType::Rook, Color::Black)),
                ((2, 8), Piece(PieceType::Knight, Color::Black)),
                ((3, 8), Piece(PieceType::Bishop, Color::Black)),
                ((4, 8), Piece(PieceType::Queen, Color::Black)),
                ((5, 8), Piece(PieceType::King, Color::Black)),
                ((6, 8), Piece(PieceType::Bishop, Color::Black)),
                ((7, 8), Piece(PieceType::Knight, Color::Black)),
                ((8, 8), Piece(PieceType::Rook, Color::Black)),
                ((1, 7), Piece(PieceType::Pawn, Color::Black)),
                ((2, 7), Piece(PieceType::Pawn, Color::Black)),
                ((3, 7), Piece(PieceType::Pawn, Color::Black)),
                ((4, 7), Piece(PieceType::Pawn, Color::Black)),
                ((5, 7), Piece(PieceType::Pawn, Color::Black)),
                ((6, 7), Piece(PieceType::Pawn, Color::Black)),
                ((7, 7), Piece(PieceType::Pawn, Color::Black)),
                ((8, 7), Piece(PieceType::Pawn, Color::Black)),
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
        let other_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let (mut to_remove, mut to_insert) = (vec![], vec![]);
        let is_castle = match special {
            Some(castle @ Special::LongCastle | castle @ Special::Castle) => {
                if piece != PieceType::King {
                    return Err(ChessError::InvalidMove);
                }
                let rook_col = match castle {
                    Special::LongCastle => 1,
                    Special::Castle => 8,
                    _ => unreachable!(),
                };
                let home_row = match color {
                    Color::White => 1,
                    Color::Black => 8,
                };
                let (from_king, from_rook) = ((5, home_row), (rook_col, home_row));
                if self.pieces.get(&from_king).is_none() || self.pieces.get(&from_rook).is_none() {
                    return Err(ChessError::InvalidMove);
                }
                let (to_king, to_rook) = match castle {
                    Special::LongCastle => ((3, home_row), (4, home_row)),
                    Special::Castle => ((7, home_row), (6, home_row)),
                    _ => unreachable!(),
                };
                if self.pieces.get(&to_king).is_some() || self.pieces.get(&to_rook).is_some() {
                    return Err(ChessError::InvalidMove);
                }
                let range = match castle {
                    Special::LongCastle => 2..5,
                    Special::Castle => 6..8,
                    _ => unreachable!(),
                };
                for col in range {
                    if self.pieces.get(&(col, home_row)).is_some() {
                        return Err(ChessError::InvalidMove);
                    }
                }
                to_remove.push(from_king);
                to_remove.push(from_rook);
                to_insert.push(to_king);
                to_insert.push(to_rook);
                true
            }
            _ => { false }
        };
        if !is_castle {
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
                        (*color == Color::White && to.1 == 4) ||
                        (*color == Color::Black && to.1 == 5)
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
                                (Some(x), Some(y)) if *x <= 8 && *y <= 8 => Some((*x, *y)),
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
                if !is_castle {
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
                                    (Some(x), Some(y)) if *x <= 8 && *y <= 8 => Some((*x, *y)),
                                    _ => None,
                                }
                            })
                            .collect()
                    );
                }
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
                let mut i = 1;
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
                            to_remove.push(coords);
                            to_insert.push(to);
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
                        to_remove.push(coords);
                        to_insert.push(to);
                        break;
                    }
                    _ => {}
                }
            }
        }
        if !piece_found && !is_castle {
            return Err(ChessError::InvalidMove);
        }

        for (from, to) in to_remove.iter().zip(to_insert.iter()) {
            let piece = self.pieces.remove(from);
            self.pieces.insert(*to, piece.unwrap());
        }

        if self.is_check(other_color) {
            println!("Check! {:?}", self.is_check(other_color));
            if let Some(Special::Check | Special::Checkmate) = special {
                self.state = GameState::Check(other_color);
            } else {
                return Err(ChessError::InvalidMove);
            }
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

    pub fn is_check(&self, color_in_check: Color) -> bool {
        let ((king_x, king_y), _) = self.pieces
            .iter()
            .find(|(_, piece)| piece.0 == PieceType::King && piece.1 == color_in_check)
            .unwrap();
        let attacking_color = match color_in_check {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        for ((piece_x, piece_y), Piece(piece_type, _color)) in self.pieces
            .iter()
            .filter(|(_, piece)| piece.1 == attacking_color) {
            if
                self.can_piece_reach(
                    (*piece_x, *piece_y),
                    (*king_x, *king_y),
                    *piece_type,
                    attacking_color
                )
            {
                return true;
            }
        }
        false
    }

    fn can_piece_reach(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        piece_type: PieceType,
        color: Color
    ) -> bool {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        let traverse = || {
            let (direction_x, direction_y) = (
                to_x.cmp(&from_x) as isize,
                to_y.cmp(&from_y) as isize,
            );
            let mut i = 1;
            loop {
                let coords = match next_coords((from_x, from_y), (direction_x, direction_y), i) {
                    Some(coords) => coords,
                    None => {
                        break;
                    }
                };
                if coords == (to_x, to_y) {
                    return true;
                }
                if self.pieces.contains_key(&coords) {
                    break;
                }
                i += 1;
            }
            return false;
        };
        match piece_type {
            PieceType::Pawn => {
                if
                    to_x.abs_diff(from_x) == 1 &&
                    to_y == (if color == Color::White { from_y + 1 } else { from_y - 1 })
                {
                    return true;
                } else {
                    return false;
                }
            }
            PieceType::Knight => {
                if
                    (to_x.abs_diff(from_x) == 2 && to_y.abs_diff(from_y) == 1) ||
                    (to_x.abs_diff(from_x) == 1 && to_y.abs_diff(from_y) == 2)
                {
                    return true;
                } else {
                    return false;
                }
            }
            PieceType::King => {
                if
                    (to_x.abs_diff(from_x) == 1 && to_y.abs_diff(from_y) == 0) ||
                    (to_x.abs_diff(from_x) == 0 && to_y.abs_diff(from_y) == 1) ||
                    (to_x.abs_diff(from_x) == 1 && to_y.abs_diff(from_y) == 1)
                {
                    return true;
                } else {
                    return false;
                }
            }
            PieceType::Queen => {
                if to_x != from_x && to_y != from_y {
                    if to_x.abs_diff(from_x) != to_y.abs_diff(from_y) {
                        return false;
                    }
                }
            }
            PieceType::Rook => {
                if to_x != from_x && to_y != from_y {
                    return false;
                }
            }
            PieceType::Bishop => {
                if to_x.abs_diff(from_x) != to_y.abs_diff(from_y) {
                    return false;
                }
            }
            _ => {
                unreachable!();
            }
        }

        traverse()
    }
}

fn notation_to_coords(notation: &str) -> Option<(usize, usize)> {
    let mut chars = notation.chars();
    let x = (chars.next().unwrap() as usize) - ('a' as usize) + 1;
    let y = (chars.next().unwrap() as usize) - ('1' as usize) + 1;
    if x > 8 || y > 8 {
        return None;
    }
    Some((x, y))
}

fn letter_to_column_index(letter: char) -> usize {
    let letter = letter.to_ascii_lowercase();
    if letter < 'a' || letter > 'h' {
        panic!("How did we get here? I thought we checked this already.");
    }
    (letter as usize) - ('a' as usize) + 1
}

fn coords_to_notation(coords: (usize, usize)) -> String {
    let x = (coords.0 as u8) + ('a' as u8) - 1;
    let y = (coords.1 as u8) + ('1' as u8) - 1;
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
    let (x, y) = origin;
    let (direction_x, direction_y) = direction;
    let x = (x as isize) + direction_x * step;
    let y = (y as isize) + direction_y * step;
    if x < 1 || x > 8 || y < 1 || y > 8 {
        return None;
    }
    Some((x as usize, y as usize))
}

fn coords_between(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    let (direction_x, direction_y) = (from_x.cmp(&to_x) as isize, from_y.cmp(&to_y) as isize);
    let mut i = 1;
    let mut coords = vec![];
    while from_x != to_x || from_y != to_y {
        coords.push(match next_coords((from_x, from_y), (direction_x, direction_y), i) {
            Some(coords) => coords,
            None => {
                break;
            }
        });
        i += 1;
    }
    coords
}