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
    
}