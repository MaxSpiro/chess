use chess::{ Game, Command };

fn main() {
    let mut chess = Game::new();
    let mut input = String::new();
    loop {
        println!("It is {:?}'s turn, enter a move", &chess.turn);

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(command) = Command::parse(&input.trim()) {
                    let result = chess.next(command);
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
    use chess::{ ChessError, PieceType, Special };

    use super::*;

    #[test]
    fn pawn_forward_capture() {
        let commands = ["e4", "e5", "exe5"];
        let expected_outputs = [Ok(()), Ok(()), Err(ChessError::InvalidMove)];
        let mut game = Game::new();
        for i in 0..3 {
            let command = Command::parse(commands[i]).unwrap();
            let result = game.next(command);
            assert_eq!(result, expected_outputs[i]);
        }
    }

    #[test]
    fn pawn_moves() {
        let mut game = Game::new();

        let commands = [
            "e4",
            "e5",
            "d4",
            "d5",
            "c3",
            "c6",
            "c4",
            "c5",
            "c5",
            "h4",
            "a3",
            "a4",
            "a5",
        ];
        let expected_outputs = [
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Err(ChessError::InvalidMove),
            Ok(()),
            Err(ChessError::InvalidMove),
            Err(ChessError::InvalidMove),
            Ok(()),
        ];
        for i in 0..13 {
            let command = Command::parse(commands[i]).unwrap();
            let result = game.next(command);
            assert_eq!(result, expected_outputs[i]);
        }

        game = Game::new();
        let commands = [
            "e4",
            "e5",
            "d4",
            "exd4",
            "c3",
            "dxc3",
            "a2",
            "a5",
            "a4",
            "b5",
            "axb5",
            "a3",
            "a6",
            "b6",
        ];
        let expected_outputs = [
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
            Err(ChessError::InvalidMove),
            Err(ChessError::InvalidMove),
            Ok(()),
            Ok(()),
            Ok(()), // this errors for some reason
            Err(ChessError::InvalidMove),
            Ok(()),
            Ok(()),
        ];

        for i in 0..13 {
            let command = Command::parse(commands[i]).unwrap();
            let result = game.next(command);
            println!("{i}");
            assert_eq!(result, expected_outputs[i]);
        }
    }

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

        assert!(Command::parse("aa4").is_none());
        assert!(Command::parse("Bax7").is_none());
        assert!(Command::parse("Rxh8+#").is_none());
        assert!(Command::parse("a").is_none());
        assert!(Command::parse("BRh8").is_none());
        assert!(Command::parse("hB8").is_none());
        assert!(Command::parse("axd9").is_none());
        assert!(Command::parse("Bj9").is_none());
        assert!(Command::parse("Ld8").is_none());
        assert!(Command::parse("4a").is_none());
    }
}