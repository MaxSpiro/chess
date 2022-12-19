use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Castle {
    KingSide,
    QueenSide,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Check {
    Check,
    Checkmate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Checkmate(Color),
    Check(Color),
    Stalemate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub fn letter(&self) -> char {
        match self.piece_type {
            PieceType::Knight => 'N',
            PieceType::Pawn => 'P',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::King => 'K',
        }
    }

    pub fn get_possible_moves(
        &self,
        piece_coords: (usize, usize),
        pieces_on_board: &HashMap<(usize, usize), Self>
    ) -> Vec<Command> {
        let (piece_x, piece_y) = piece_coords;
        let from = (Some(piece_x), Some(piece_y));
        let command_builder = CommandBuilder::new().from(from).piece(self.piece_type);
        let mut moves = vec![];
        match self.piece_type {
            PieceType::Pawn => {
                let pawn_row = match self.color {
                    Color::White => 2,
                    Color::Black => 7,
                };
                let pawn_steps = if piece_y == pawn_row { 1..3 } else { 1..2 };
                for step in pawn_steps {
                    if let Some(new_y) = pawn_move(piece_y, step, self.color) {
                        if pieces_on_board.get(&(piece_x, new_y)).is_none() {
                            moves.push(command_builder.to((piece_x, new_y)).build());
                        }
                        // can also calculate capture when step is 1
                        if step == 1 {
                            for possible_capture in [piece_x.checked_sub(1), piece_x.checked_add(1)]
                                .into_iter()
                                .filter_map(|optional_coord| {
                                    if let Some(x_coord) = optional_coord {
                                        if x_coord > 0 && x_coord <= 8 {
                                            return Some((x_coord, new_y));
                                        }
                                    }
                                    None
                                }) {
                                match pieces_on_board.get(&possible_capture) {
                                    Some(piece) if piece.color != self.color => {
                                        moves.push(
                                            command_builder.takes(true).to(possible_capture).build()
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            PieceType::King | PieceType::Knight => {
                for possible_coords in self.get_candidate_moves(piece_coords) {
                    if let (Some(x), Some(y)) = possible_coords {
                        if x < 1 || x > 8 || y < 1 || y > 8 {
                            continue;
                        }
                        let takes;
                        match pieces_on_board.get(&(x, y)) {
                            Some(piece) if piece.color != self.color => {
                                takes = true;
                            }
                            None => {
                                takes = false;
                            }
                            _ => {
                                continue;
                            }
                        }
                        moves.push(command_builder.to((x, y)).takes(takes).build());
                    }
                }
            }
            PieceType::Bishop | PieceType::Queen | PieceType::Rook => {
                let directions = self.get_direction_vectors();
                for direction in directions {
                    let mut step = 1;
                    loop {
                        if let Some(next_coords) = next_coords(piece_coords, direction, step) {
                            let takes;
                            match pieces_on_board.get(&next_coords) {
                                Some(piece) => {
                                    if piece.color == self.color {
                                        break;
                                    } else {
                                        takes = true;
                                    }
                                }
                                None => {
                                    takes = false;
                                }
                            }
                            moves.push(command_builder.takes(takes).to(next_coords).build());
                        } else {
                            break;
                        }
                        step += 1;
                    }
                }
            }
        }
        moves
            .into_iter()
            .map(|command| {
                let game = Game::from(pieces_on_board.clone(), self.color);
                match game.simulate_move(&command) {
                    Ok(_) => {
                        match game.state {
                            GameState::Checkmate(_) => {
                                Command {
                                    check: Some(Check::Checkmate),
                                    ..command
                                }
                            }
                            GameState::Check(_) => {
                                Command {
                                    check: Some(Check::Check),
                                    ..command
                                }
                            }
                            _ => { command }
                        }
                    }
                    Err(_) => { command }
                }
            })
            .collect()
    }

    fn can_move(
        &self,
        piece_coords: (usize, usize),
        target_coords: (usize, usize),
        pieces_on_board: &HashMap<(usize, usize), Self>,
        takes: bool
    ) -> bool {
        let (from_x, from_y) = piece_coords;
        let (to_x, to_y) = target_coords;
        match self.piece_type {
            PieceType::Pawn => {
                match takes {
                    true => {
                        if
                            to_x.abs_diff(from_x) == 1 &&
                            to_y ==
                                (if self.color == Color::White { from_y + 1 } else { from_y - 1 })
                        {
                            return true;
                        } else {
                            return false;
                        }
                    }
                    false => {
                        if
                            to_x == from_x &&
                            (to_y == pawn_move(from_y, 1, self.color).unwrap_or_default() ||
                                (from_y ==
                                    (match self.color {
                                        Color::White => 2,
                                        Color::Black => 7,
                                    }) &&
                                    to_y == pawn_move(from_y, 2, self.color).unwrap_or_default()))
                        {
                            return true;
                        } else {
                            return false;
                        }
                    }
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
                if
                    to_x != from_x &&
                    to_y != from_y &&
                    to_x.abs_diff(from_x) != to_y.abs_diff(from_y)
                {
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
        }

        let (direction_x, direction_y) = (to_x.cmp(&from_x) as isize, to_y.cmp(&from_y) as isize);
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
            if pieces_on_board.contains_key(&coords) {
                break;
            }
            i += 1;
        }
        false
    }

    fn get_direction_vectors(&self) -> Vec<(isize, isize)> {
        match self.piece_type {
            PieceType::Bishop => vec![(1, 1), (1, -1), (-1, 1), (-1, -1)],
            PieceType::Rook => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            PieceType::Queen =>
                vec![(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1)],
            _ => panic!("Only bishops, rooks, and queens move with a direction vector"),
        }
    }

    fn get_candidate_moves(
        &self,
        piece_coords: (usize, usize)
    ) -> Vec<(Option<usize>, Option<usize>)> {
        let (piece_x, piece_y) = piece_coords;
        match self.piece_type {
            PieceType::Knight => {
                vec![
                    (piece_x.checked_add(1), piece_y.checked_add(2)),
                    (piece_x.checked_add(1), piece_y.checked_sub(2)),
                    (piece_x.checked_sub(1), piece_y.checked_add(2)),
                    (piece_x.checked_sub(1), piece_y.checked_sub(2)),
                    (piece_x.checked_add(2), piece_y.checked_add(1)),
                    (piece_x.checked_add(2), piece_y.checked_sub(1)),
                    (piece_x.checked_sub(2), piece_y.checked_add(1)),
                    (piece_x.checked_sub(2), piece_y.checked_sub(1))
                ]
            }
            PieceType::King => {
                vec![
                    (piece_x.checked_add(1), piece_y.checked_add(1)),
                    (piece_x.checked_add(1), piece_y.checked_sub(1)),
                    (piece_x.checked_sub(1), piece_y.checked_add(1)),
                    (piece_x.checked_sub(1), piece_y.checked_sub(1)),
                    (piece_x.checked_add(1), Some(piece_y)),
                    (piece_x.checked_sub(1), Some(piece_y)),
                    (Some(piece_x), piece_y.checked_add(1)),
                    (Some(piece_x), piece_y.checked_sub(1))
                ]
            }
            _ => panic!("Only call this method on a king or knight"),
        }
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
    pub piece: PieceType,
    pub from: (Option<usize>, Option<usize>),
    pub to: (usize, usize),
    pub takes: bool,
    pub check: Option<Check>,
    pub castle: Option<Castle>,
}

#[derive(Copy, Clone)]
pub struct CommandBuilder {
    piece: Option<PieceType>,
    from: Option<(Option<usize>, Option<usize>)>,
    to: Option<(usize, usize)>,
    takes: Option<bool>,
    check: Option<Check>,
    castle: Option<Castle>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self {
            piece: None,
            from: None,
            to: None,
            takes: None,
            check: None,
            castle: None,
        }
    }

    pub fn piece(mut self, piece: PieceType) -> Self {
        self.piece = Some(piece);
        self
    }

    pub fn from(mut self, from: (Option<usize>, Option<usize>)) -> Self {
        self.from = Some(from);
        self
    }

    pub fn to(mut self, to: (usize, usize)) -> Self {
        self.to = Some(to);
        self
    }

    pub fn takes(mut self, takes: bool) -> Self {
        self.takes = Some(takes);
        self
    }

    pub fn castle(mut self, castle: Option<Castle>) -> Self {
        self.castle = castle;
        self
    }

    pub fn check(mut self, check: Option<Check>) -> Self {
        self.check = check;
        self
    }

    pub fn build(self) -> Command {
        Command {
            piece: self.piece.unwrap(),
            to: self.to.unwrap_or((0, 0)),
            from: self.from.unwrap_or((None, None)),
            takes: self.takes.unwrap_or(false),
            check: self.check,
            castle: self.castle,
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
        let command_builder = CommandBuilder::new();
        let captures = NOTATION_PATTERN.captures(input)?;
        if input == "O-O" {
            return Some(
                command_builder.piece(PieceType::King).castle(Some(Castle::KingSide)).build()
            );
        }
        if input == "O-O-O" {
            return Some(
                command_builder.piece(PieceType::King).castle(Some(Castle::QueenSide)).build()
            );
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
        let check = match captures.name("check") {
            Some(check) => {
                match check.as_str() {
                    "+" => Some(Check::Check),
                    "#" => Some(Check::Checkmate),
                    _ => None,
                }
            }
            None => None,
        };

        Some(
            command_builder
                .to(notation_to_coords(to).unwrap())
                .piece(piece)
                .from((from_col, from_row))
                .takes(takes)
                .check(check)
                .build()
        )
    }

    pub fn to_notation(&self) -> String {
        let suffix = match self.check {
            Some(Check::Check) => "+",
            Some(Check::Checkmate) => "#",
            None => "",
        };
        match self.castle {
            Some(Castle::KingSide) => {
                return format!("O-O{}", suffix);
            }
            Some(Castle::QueenSide) => {
                return format!("O-O-O,{}", suffix);
            }
            _ => {}
        }
        let mut notation = String::new();

        if self.piece != PieceType::Pawn {
            notation.push(match self.piece {
                PieceType::Bishop => 'B',
                PieceType::King => 'K',
                PieceType::Knight => 'N',
                PieceType::Queen => 'Q',
                PieceType::Rook => 'R',
                _ => unreachable!(),
            });
        } else {
            if self.takes {
                notation.push(
                    column_index_to_letter(
                        self.from.0.expect("Must have a-h specified for pawn takes")
                    )
                );
            }
        }
        if self.takes {
            notation.push('x');
        }
        notation.push_str(coords_to_notation(self.to).as_str());
        notation.push_str(suffix);
        notation
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChessError {
    InvalidMove,
    InCheck,
}

use std::fmt::{ Display, Formatter };

impl std::error::Error for ChessError {}

impl Display for ChessError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ChessError::InvalidMove => write!(f, "Invalid move"),
            ChessError::InCheck =>
                write!(
                    f,
                    "Cannot move into check. If you are in check, you must move out of check"
                ),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        board.push_str("-".repeat(17).as_str());
        board.push_str("\n");
        for row in 1..=8 {
            board.push('|');
            for col in 1..=8 {
                board.push(match self.pieces.get(&(col, 9 - row)) {
                    Some(piece) => piece.letter(),
                    None => ' ',
                });
                board.push('|');
            }
            board.push_str("\n");
            board.push_str("-".repeat(17).as_str());
            board.push_str("\n");
        }
        writeln!(f, "{}", board)
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Color::White,
            pieces: [
                ((1, 1), Piece { piece_type: PieceType::Rook, color: Color::White }),
                ((2, 1), Piece { piece_type: PieceType::Knight, color: Color::White }),
                ((3, 1), Piece { piece_type: PieceType::Bishop, color: Color::White }),
                ((4, 1), Piece { piece_type: PieceType::Queen, color: Color::White }),
                ((5, 1), Piece { piece_type: PieceType::King, color: Color::White }),
                ((6, 1), Piece { piece_type: PieceType::Bishop, color: Color::White }),
                ((7, 1), Piece { piece_type: PieceType::Knight, color: Color::White }),
                ((8, 1), Piece { piece_type: PieceType::Rook, color: Color::White }),
                ((1, 2), Piece { piece_type: PieceType::Pawn, color: Color::White }),
                ((2, 2), Piece { piece_type: PieceType::Pawn, color: Color::White }),
                ((3, 2), Piece { piece_type: PieceType::Pawn, color: Color::White }),
                ((4, 2), Piece { piece_type: PieceType::Pawn, color: Color::White }),
                ((5, 2), Piece { piece_type: PieceType::Pawn, color: Color::White }),
                ((6, 2), Piece { piece_type: PieceType::Pawn, color: Color::White }),
                ((7, 2), Piece { piece_type: PieceType::Pawn, color: Color::White }),
                ((8, 2), Piece { piece_type: PieceType::Pawn, color: Color::White }),
                ((1, 8), Piece { piece_type: PieceType::Rook, color: Color::Black }),
                ((2, 8), Piece { piece_type: PieceType::Knight, color: Color::Black }),
                ((3, 8), Piece { piece_type: PieceType::Bishop, color: Color::Black }),
                ((4, 8), Piece { piece_type: PieceType::Queen, color: Color::Black }),
                ((5, 8), Piece { piece_type: PieceType::King, color: Color::Black }),
                ((6, 8), Piece { piece_type: PieceType::Bishop, color: Color::Black }),
                ((7, 8), Piece { piece_type: PieceType::Knight, color: Color::Black }),
                ((8, 8), Piece { piece_type: PieceType::Rook, color: Color::Black }),
                ((1, 7), Piece { piece_type: PieceType::Pawn, color: Color::Black }),
                ((2, 7), Piece { piece_type: PieceType::Pawn, color: Color::Black }),
                ((3, 7), Piece { piece_type: PieceType::Pawn, color: Color::Black }),
                ((4, 7), Piece { piece_type: PieceType::Pawn, color: Color::Black }),
                ((5, 7), Piece { piece_type: PieceType::Pawn, color: Color::Black }),
                ((6, 7), Piece { piece_type: PieceType::Pawn, color: Color::Black }),
                ((7, 7), Piece { piece_type: PieceType::Pawn, color: Color::Black }),
                ((8, 7), Piece { piece_type: PieceType::Pawn, color: Color::Black }),
            ]
                .iter()
                .cloned()
                .collect::<HashMap<(usize, usize), Piece>>(),
            state: GameState::InProgress,
        }
    }

    pub fn from(pieces: HashMap<(usize, usize), Piece>, turn: Color) -> Game {
        Game {
            pieces,
            turn,
            state: GameState::InProgress,
        }
    }

    pub fn simulate_move(&self, input: &Command) -> Result<Self, ChessError> {
        let mut new_board = self.clone();
        let Command { to, from, piece, takes, castle, .. } = input;
        let Game { turn: color, .. } = new_board;

        match self.pieces.get(&to) {
            Some(_) => {
                if !takes {
                    return Err(ChessError::InvalidMove);
                }
            }
            None => {
                if *takes {
                    return Err(ChessError::InvalidMove);
                }
            }
        }

        if let Some(castle) = castle {
            if piece != &PieceType::King {
                return Err(ChessError::InvalidMove);
            }
            let rook_col = match castle {
                Castle::QueenSide => 1,
                Castle::KingSide => 8,
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
                Castle::QueenSide => ((3, home_row), (4, home_row)),
                Castle::KingSide => ((7, home_row), (6, home_row)),
                _ => unreachable!(),
            };
            if self.pieces.get(&to_king).is_some() || self.pieces.get(&to_rook).is_some() {
                return Err(ChessError::InvalidMove);
            }
            let range = match castle {
                Castle::QueenSide => 2..5,
                Castle::KingSide => 6..8,
                _ => unreachable!(),
            };
            for col in range {
                if self.pieces.get(&(col, home_row)).is_some() {
                    return Err(ChessError::InvalidMove);
                }
            }
            let king = new_board.pieces.remove(&from_king).unwrap();
            let rook = new_board.pieces.remove(&from_rook).unwrap();
            new_board.pieces.insert(to_king, king);
            new_board.pieces.insert(to_rook, rook);
        } else {
            for (coords, candidate_piece) in self.pieces
                .iter()
                .filter(|(coords, p)| {
                    coords_match_from(**coords, *from) && p.piece_type == *piece && p.color == color
                }) {
                if candidate_piece.can_move(*coords, *to, &self.pieces, *takes) {
                    new_board.pieces.remove(&to);
                    new_board.pieces.insert(*to, candidate_piece.clone());
                    new_board.pieces.remove(coords);
                }
            }
        }

        if new_board.is_check(new_board.turn) {
            return Err(ChessError::InCheck);
        }

        Ok(new_board)
    }

    pub fn play(&mut self, command: &Command) -> Result<(), ChessError> {
        let new_game = self.simulate_move(command)?;

        *self = new_game;
        self.next_turn();

        let is_check = self.is_check(self.turn);
        let moves = self.get_all_possible_moves(self.turn);

        if is_check && moves.len() == 0 {
            self.state = GameState::Checkmate(self.turn.opposite());
        } else if is_check && moves.len() != 0 {
            self.state = GameState::Check(self.turn);
        } else if !is_check && moves.len() == 0 {
            self.state = GameState::Stalemate;
        }

        Ok(())
    }

    fn next_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    // these are both expensive calculations and should be cached or called less often
    pub fn is_checkmate(&self, color_in_check: Color) -> bool {
        self.is_check(color_in_check) && self.get_all_possible_moves(color_in_check).len() == 0
    }

    pub fn is_stalemate(&self, color_in_check: Color) -> bool {
        !self.is_check(color_in_check) && self.get_all_possible_moves(color_in_check).len() == 0
    }

    pub fn is_check(&self, color_in_check: Color) -> bool {
        let king_coords = match
            self.pieces
                .iter()
                .find(
                    |(_, piece)|
                        piece.piece_type == PieceType::King && piece.color == color_in_check
                )
        {
            Some((coords, _)) => coords,
            None => {
                return false;
            }
        };
        let attacking_color = match color_in_check {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        for (piece_coords, piece) in self.pieces
            .iter()
            .filter(|(_, piece)| piece.color == attacking_color) {
            if piece.can_move(*piece_coords, *king_coords, &self.pieces, true) {
                return true;
            }
        }
        false
    }

    pub fn get_all_possible_moves(&self, color: Color) -> Vec<Command> {
        self.pieces
            .iter()
            .filter(|(_, Piece { color: _color, .. })| { _color == &color })
            .flat_map(|(coords, piece)| { piece.get_possible_moves(*coords, &self.pieces) })
            .filter(|command| { self.simulate_move(&command).is_ok() })
            .collect()
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

fn column_index_to_letter(col: usize) -> char {
    if col < 1 || col > 8 {
        panic!();
    }
    let char = col - 1 + ('a' as usize);
    char as u8 as char
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

fn pawn_move(y_coord: usize, step: isize, color: Color) -> Option<usize> {
    let direction = if color == Color::White { 1 } else { -1 };
    let new_y = (y_coord as isize) + step * direction;

    if new_y < 1 || new_y > 8 {
        None
    } else {
        Some(new_y as usize)
    }
}