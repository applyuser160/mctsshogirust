#[cfg(test)]

mod tests {
    use crate::{board::Board, color::ColorType, game::Game};


    #[test]
    fn case01() {
        let sfen = String::from("startpos");
        let mut game = Game::new();
        game.input_board(sfen);
        let mut board = Board::new();
        board.startpos();
        assert_eq!(game.board, board);
        assert_eq!(game.move_number, 1);
        assert_eq!(game.turn, ColorType::Black);
        assert_eq!(game.winner, ColorType::None);
    }

    #[test]
    fn case02() {
        let sfen1 = String::from("startpos");
        let mut game1 = Game::new();
        game1.input_board(sfen1);
        let sfen_str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL";
        let sfen2 = String::from(sfen_str);
        let mut game2 = Game::new();
        game2.input_board(sfen2);
        assert_eq!(game1.board.to_string(), game2.board.to_string());
        assert_eq!(sfen_str, game1.board.to_string());
    }
    
}