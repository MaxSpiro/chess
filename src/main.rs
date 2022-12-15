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
                    println!("{:?}", result);
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
    use chess::{ ChessError, Color, GameState, Piece, PieceType, Special };

    use super::*;

    #[test]
    fn commands() {
        let command = Command::parse("Kd4").unwrap();
        assert_eq!(command.piece, PieceType::King);
        assert_eq!(command.to, (3, 3));
        assert_eq!(command.takes, false);

        let command = Command::parse("Qd4").unwrap();
        assert_eq!(command.piece, PieceType::Queen);
        assert_eq!(command.to, (3, 3));
        assert_eq!(command.takes, false);

        let command = Command::parse("Rxa8").unwrap();
        assert_eq!(command.piece, PieceType::Rook);
        assert_eq!(command.to, (0, 7));
        assert_eq!(command.takes, true);

        let command = Command::parse("a4").unwrap();
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (0, 3));
        assert_eq!(command.takes, false);

        let command = Command::parse("axd4").unwrap();
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (3, 3));
        assert_eq!(command.takes, true);

        // write me some more tests for more moves like O-O-O, Bxh8, axd3, etc.
        let command = Command::parse("Bxh8").unwrap();
        assert_eq!(command.piece, PieceType::Bishop);
        assert_eq!(command.to, (7, 7));
        assert_eq!(command.takes, true);

        let command = Command::parse("axd3").unwrap();
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (3, 2));
        assert_eq!(command.takes, true);

        let command = Command::parse("O-O-O").unwrap();
        assert_eq!(command.piece, PieceType::King);
        assert_eq!(command.special, Some(Special::LongCastle));

        let command = Command::parse("d4+").unwrap();
        assert_eq!(command.piece, PieceType::Pawn);
        assert_eq!(command.to, (3, 3));
        assert_eq!(command.takes, false);
        assert_eq!(command.special, Some(Special::Check));

        let command = Command::parse("Ba3#").unwrap();
        assert_eq!(command.piece, PieceType::Bishop);
        assert_eq!(command.to, (0, 2));
        assert_eq!(command.takes, false);
        assert_eq!(command.special, Some(Special::Checkmate));

        let command = Command::parse("Qxh8+").unwrap();
        assert_eq!(command.piece, PieceType::Queen);
        assert_eq!(command.to, (7, 7));
        assert_eq!(command.takes, true);
        assert_eq!(command.special, Some(Special::Check));

        let command = Command::parse("Rexa8#").unwrap();
        assert_eq!(command.from, (Some(4), None));
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
            (0, 5),
            (1, 2),
            (1, 4),
            (2, 2),
            (2, 5),
            (3, 4),
            (4, 3),
            (4, 4),
            (4, 6),
            (5, 5),
            (4, 0),
            (6, 0),
            (5, 7),
            (6, 7),
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

        assert_eq!(chess.pieces.get(&(4, 4)), Some(&Piece::new(PieceType::Pawn, Color::Black)));
        assert_eq!(chess.pieces.get(&(4, 1)), Some(&Piece::new(PieceType::King, Color::White)));
        assert_eq!(chess.pieces.get(&(4, 6)), Some(&Piece::new(PieceType::King, Color::Black)));
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
        let result = chess.play(Command::parse("Qxf7#").unwrap());
        assert_eq!(result, Ok(()));

        assert_eq!(chess.pieces.get(&(5, 6)), Some(&Piece::new(PieceType::Queen, Color::White)));
        assert_eq!(chess.pieces.get(&(2, 3)), Some(&Piece::new(PieceType::Bishop, Color::White)));

        assert_eq!(chess.state, GameState::Checkmate(Color::White));
    }
}