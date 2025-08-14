#[cfg(test)]
mod tests {
    use chess_tui::app::App;
    use chess_tui::game_logic::board::Board;
    use chess_tui::game_logic::game_board::GameBoard;
    use chess_tui::pieces::{PieceColor, PieceType};

    #[test]
    pub fn test_board_vanilla() {
        let app = App::default();

        assert_eq!(app.game.game_board.board, GameBoard::default().board);
    }

    #[test]
    pub fn test_board_modify() {
        let mut app = App::default();
        app.chess960_board();

        assert_ne!(app.game.game_board.board, GameBoard::default().board);
    }

    #[test]
    pub fn test_valid_king() {
        let mut app = App::default();
        app.chess960_board();

        let board: Board = app.game.game_board.board;

        assert_ne!(board[0][0].unwrap().0, PieceType::King);
        assert_ne!(board[0][7].unwrap().0, PieceType::King);
        assert_ne!(board[7][7].unwrap().0, PieceType::King);
        assert_ne!(board[7][7].unwrap().0, PieceType::King);
    }

    #[test]
    pub fn test_valid_bishop_white() {
        let mut app = App::default();
        app.chess960_board();

        let board: Board = app.game.game_board.board;
        let mut first: Option<usize> = None;

        for column in board.iter() {
            for (index, piece) in column.iter().enumerate() {
                if is_bishop_white(piece) {
                    if first.is_none() {
                        first = Some(index);
                    } else {
                        assert_ne!(first.unwrap() % 2, index % 2);
                    }
                }
            }
        }
    }

    #[test]
    pub fn test_valid_bishop_black() {
        let mut app = App::default();
        app.chess960_board();

        let board: Board = app.game.game_board.board;
        let mut first: Option<usize> = None;

        for column in board.iter() {
            for (index, piece) in column.iter().enumerate() {
                if is_bishop_black(piece) {
                    if first.is_none() {
                        first = Some(index);
                    } else {
                        assert_ne!(first.unwrap() % 2, index % 2);
                    }
                }
            }
        }
    }

    pub fn is_bishop_white(piece: &Option<(PieceType, PieceColor)>) -> bool {
        if piece.is_none() {
            return false;
        }

        if piece.unwrap().1.eq(&PieceColor::Black) {
            return false;
        }

        piece.unwrap().0.eq(&PieceType::Bishop)
    }

    pub fn is_bishop_black(piece: &Option<(PieceType, PieceColor)>) -> bool {
        if piece.is_none() {
            return false;
        }

        if piece.unwrap().1.eq(&PieceColor::White) {
            return false;
        }

        piece.unwrap().0.eq(&PieceType::Bishop)
    }

}