use chess::{ Command, Game };

fn main() {
    let mut chess = Game::new();
    let mut input = String::new();
    loop {
        println!("It is {:?}'s turn, enter a move", &chess.turn);
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(command) = Command::parse(&input.trim()) {
                    let result = chess.play(command);
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                } else {
                    println!("Invalid move");
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        println!("\n");
        input.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chess::{ ChessError, Color, GameState, Piece, PieceType, Special };

    use super::*;

    #[test]
    fn commands_can_parse() {
        let command = Command::parse("Kd4").unwrap();
        assert_eq!(command.piece, PieceType::King);
        assert_eq!(command.to, (4, 4));
        assert_eq!(command.takes, false);

        let command = Command::parse("Qd4").unwrap();
        assert_eq!(command.piece, PieceType::Queen);
        assert_eq!(command.to, (4, 4));
        assert_eq!(command.takes, false);

        let command = Command::parse("Rxa8").unwrap();
        assert_eq!(command.piece, PieceType::Rook);
        assert_eq!(command.to, (1, 8));
        assert_eq!(command.takes, true);

        let command = Command::parse("a4").unwrap();
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (1, 4));
        assert_eq!(command.takes, false);

        let command = Command::parse("axd4").unwrap();
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (4, 4));
        assert_eq!(command.takes, true);

        // write me some more tests for more moves like O-O-O, Bxh8, axd3, etc.
        let command = Command::parse("Bxh8").unwrap();
        assert_eq!(command.piece, PieceType::Bishop);
        assert_eq!(command.to, (8, 8));
        assert_eq!(command.takes, true);

        let command = Command::parse("axd3").unwrap();
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (4, 3));
        assert_eq!(command.takes, true);

        let command = Command::parse("O-O-O").unwrap();
        assert_eq!(command.piece, PieceType::King);
        assert_eq!(command.special, Some(Special::LongCastle));

        let command = Command::parse("d4+").unwrap();
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (4, 4));
        assert_eq!(command.takes, false);
        assert_eq!(command.special, Some(Special::Check));

        let command = Command::parse("Ba3#").unwrap();
        assert_eq!(command.piece, PieceType::Bishop);
        assert_eq!(command.to, (1, 3));
        assert_eq!(command.takes, false);
        assert_eq!(command.special, Some(Special::Checkmate));

        let command = Command::parse("Qxh8+").unwrap();
        assert_eq!(command.piece, PieceType::Queen);
        assert_eq!(command.to, (8, 8));
        assert_eq!(command.takes, true);
        assert_eq!(command.special, Some(Special::Check));

        let command = Command::parse("Rexa8#").unwrap();
        assert_eq!(command.from, (Some(5), None));
        assert_eq!(command.takes, true);
        assert_eq!(command.special, Some(Special::Checkmate));

        let invalid_commands = [
            "aa4",
            "e55",
            "O-",
            "O-O-O-O",
            "Qxd44#",
            "Qxd4+#",
            "a",
            "Qxd4x",
            "Qxd4xQ",
            "aa4",
            "Bax7",
            "Rxh8+#",
            "a",
            "BRh8",
            "hB8",
            "axd9",
            "Bj9",
            "Ld8",
            "4a",
            "Ka8xh7=Q+",
        ];
        for command in invalid_commands {
            assert!(Command::parse(command).is_none());
        }

        let valid_commands = [
            "e4",
            "Rag8",
            "N3f7",
            "Nd5",
            "dxc3",
            "Kxa8",
            "Be3+",
            "O-O",
            "O-O-O",
            "Qxd4#",
        ];
        for command in valid_commands {
            assert!(Command::parse(command).is_some());
        }
    }

    #[test]
    fn all_pieces_can_check() {
        let mut chess = Game {
            turn: Color::White,
            state: GameState::InProgress,
            pieces: vec![
                ((2, 2), Piece::new(PieceType::King, Color::White)),
                ((2, 4), Piece::new(PieceType::King, Color::Black))
            ]
                .into_iter()
                .collect(),
        };
        assert!(!chess.is_check(Color::White));
        assert!(!chess.is_check(Color::Black));

        chess.pieces.insert((2, 3), Piece::new(PieceType::Queen, Color::White));
        assert!(chess.is_check(Color::Black));
        assert!(!chess.is_check(Color::White));

        chess.pieces.insert((2, 3), Piece::new(PieceType::Queen, Color::Black));
        assert!(!chess.is_check(Color::Black));
        assert!(chess.is_check(Color::White));

        chess.pieces.remove(&(2, 3));
        chess.pieces.insert((6, 6), Piece::new(PieceType::Bishop, Color::Black));
        assert!(chess.is_check(Color::White));
        assert!(!chess.is_check(Color::Black));

        chess.pieces.remove(&(6, 6));
        chess.pieces.insert((6, 8), Piece::new(PieceType::Bishop, Color::White));
        assert!(!chess.is_check(Color::White));
        assert!(chess.is_check(Color::Black));

        chess.pieces.remove(&(6, 8));
        chess.pieces.insert((3, 3), Piece::new(PieceType::Pawn, Color::Black));
        assert!(chess.is_check(Color::White));
        chess.pieces.remove(&(3, 3));
        chess.pieces.insert((1, 3), Piece::new(PieceType::Pawn, Color::Black));
        assert!(chess.is_check(Color::White));
        chess.pieces.remove(&(1, 3));
        chess.pieces.insert((2, 3), Piece::new(PieceType::Pawn, Color::Black));
        println!("pieces:");
        for piece in &chess.pieces {
            println!("{:?}", piece);
        }
        println!("done");
        assert!(!chess.is_check(Color::White));
        chess.pieces.remove(&(2, 3));

        chess.pieces.insert((2, 8), Piece::new(PieceType::Rook, Color::Black));
        // can't check through own king
        assert!(!chess.is_check(Color::White));
        // remove king, now can check
        chess.pieces.remove(&(2, 4));
        assert!(chess.is_check(Color::White));
    }

    #[test]
    fn ruy_lopez() {
        let mut chess = Game::new();
        let moves = [
            "e4",
            "e5",
            "Nf3",
            "Nc6",
            "Bb5",
            "a6",
            "Ba4",
            "Nf6",
            "O-O",
            "Be7",
            "Re1",
            "b5",
            "Bb3",
            "O-O",
            "c3",
            "d5",
        ];

        for command in moves {
            println!("{:?}: {}", chess.turn, command);
            chess.play(Command::parse(command).unwrap()).unwrap();
        }

        let coords = [
            (1, 6),
            (2, 3),
            (2, 5),
            (3, 3),
            (3, 6),
            (4, 5),
            (5, 4),
            (5, 5),
            (5, 7),
            (6, 6),
            (5, 1),
            (7, 1),
            (6, 8),
            (7, 8),
        ];
        let pieces = [
            Piece::new(PieceType::Pawn, Color::Black),
            Piece::new(PieceType::Bishop, Color::White),
            Piece::new(PieceType::Pawn, Color::Black),
            Piece::new(PieceType::Pawn, Color::White),
            Piece::new(PieceType::Knight, Color::Black),
            Piece::new(PieceType::Pawn, Color::Black),
            Piece::new(PieceType::Pawn, Color::White),
            Piece::new(PieceType::Pawn, Color::Black),
            Piece::new(PieceType::Bishop, Color::Black),
            Piece::new(PieceType::Knight, Color::Black),
            Piece::new(PieceType::Rook, Color::White),
            Piece::new(PieceType::King, Color::White),
            Piece::new(PieceType::Rook, Color::Black),
            Piece::new(PieceType::King, Color::Black),
        ];

        for (coord, piece) in coords.iter().zip(pieces.iter()) {
            assert_eq!(chess.pieces.get(coord), Some(piece));
        }
    }

    #[test]
    fn castle_to_check() {
        let mut game = Game::new();
        let moves = [
            "e4",
            "f5",
            "exf5",
            "Nc6",
            "Nf3",
            "a6",
            "Bc4",
            "a5",
            "O-O",
            "e5",
            "d4",
            "exd4",
            "Re1+",
        ];
        for command in moves {
            game.play(Command::parse(command).unwrap()).unwrap();
        }
        assert!(game.is_check(Color::Black));
    }

    #[test]
    fn bongcloud() {
        let mut chess = Game::new();
        let result = chess.play(Command::parse("e4").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("e5").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("Ke2").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("Ke7").unwrap());
        assert_eq!(result, Ok(()));

        assert_eq!(chess.pieces.get(&(5, 5)), Some(&Piece::new(PieceType::Pawn, Color::Black)));
        assert_eq!(chess.pieces.get(&(5, 2)), Some(&Piece::new(PieceType::King, Color::White)));
        assert_eq!(chess.pieces.get(&(5, 7)), Some(&Piece::new(PieceType::King, Color::Black)));
    }

    #[test]
    fn fried_liver() {
        let mut chess = Game::new();
        let result = chess.play(Command::parse("e4").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("e5").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("Qh5").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("Nc6").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("Bc4").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("Nf6").unwrap());
        assert_eq!(result, Ok(()));
        let result = chess.play(Command::parse("Qxf7+").unwrap());
        assert_eq!(result, Ok(()));

        assert_eq!(chess.pieces.get(&(6, 7)), Some(&Piece::new(PieceType::Queen, Color::White)));
        assert_eq!(chess.pieces.get(&(5, 8)), Some(&Piece::new(PieceType::King, Color::Black)));
        assert_eq!(chess.pieces.get(&(3, 4)), Some(&Piece::new(PieceType::Bishop, Color::White)));

        assert_eq!(chess.state, GameState::Check(Color::Black));
    }
}