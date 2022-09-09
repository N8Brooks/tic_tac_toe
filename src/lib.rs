#[derive(Debug)]
pub enum TicTacToeError {
    RowOutOfBounds,
    ColOutOfBounds,
    GameConcluded,
    PositionTaken,
}

#[derive(Copy, Clone, Debug)]
pub enum Mark {
    X,
    O,
}

impl Mark {
    fn toggle(&mut self) {
        *self = if matches!(self, Mark::X) {
            Mark::O
        } else {
            Mark::X
        };
    }
}

#[derive(Debug)]
pub enum Winner {
    Player(Mark),
    Draw,
}

pub struct TicTacToeState {
    board: [[Option<Mark>; 3]; 3],
    turn: Mark,
    winner: Option<Winner>,
}

impl TicTacToeState {
    pub fn default() -> TicTacToeState {
        TicTacToeState {
            turn: Mark::X,
            winner: None,
            board: [[None; 3]; 3],
        }
    }

    pub fn place(&mut self, row: usize, col: usize) -> Result<Option<Winner>, TicTacToeError> {
        if !(0..3).contains(&row) {
            Err(TicTacToeError::RowOutOfBounds)
        } else if !(0..3).contains(&col) {
            Err(TicTacToeError::ColOutOfBounds)
        } else if self.winner.is_some() {
            Err(TicTacToeError::GameConcluded)
        } else if self.board[row][col].is_some() {
            Err(TicTacToeError::PositionTaken)
        } else {
            self.board[row][col] = Some(self.turn);
            self.turn.toggle();
            Ok(self.find_winner())
        }
    }

    fn find_winner(&mut self) -> Option<Winner> {
        let is_x = self.is_winner(Mark::X);
        let is_o = self.is_winner(Mark::O);
        if is_x && is_o {
            panic!("invalid state: x and o tied")
        } else if is_x {
            Some(Winner::Player(Mark::X))
        } else if is_o {
            Some(Winner::Player(Mark::O))
        } else if self.is_full() {
            Some(Winner::Draw)
        } else {
            None
        }
    }

    fn is_winner(&self, winner: Mark) -> bool {
        self.is_winner_forward_diagonal(winner)
            || self.is_winner_backward_diagonal(winner)
            || self.is_winner_rows(winner)
            || self.is_winner_cols(winner)
    }

    fn is_winner_forward_diagonal(&self, winner: Mark) -> bool {
        self.board
            .iter()
            .enumerate()
            .all(|(i, row)| is_some_and_matches_winner(row[i], winner))
    }

    fn is_winner_backward_diagonal(&self, winner: Mark) -> bool {
        self.board
            .iter()
            .enumerate()
            .all(|(i, row)| is_some_and_matches_winner(row[2 - i], winner))
    }

    fn is_winner_rows(&self, winner: Mark) -> bool {
        self.board.iter().any(|row| {
            row.iter()
                .all(|mark| is_some_and_matches_winner(*mark, winner))
        })
    }

    fn is_winner_cols(&self, winner: Mark) -> bool {
        (0..3).map(|j| self.board.map(|row| row[j])).any(|col| {
            col.iter()
                .all(|mark| is_some_and_matches_winner(*mark, winner))
        })
    }

    fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|mark| mark.is_some()))
    }
}

fn is_some_and_matches_winner(mark: Option<Mark>, winner: Mark) -> bool {
    match mark {
        None => false,
        Some(Mark::X) => matches!(winner, Mark::X),
        Some(Mark::O) => matches!(winner, Mark::O),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game() {
        let mut state = TicTacToeState::default();
        assert!(state.place(0, 0).expect("vacant").is_none());
        assert!(state.place(0, 1).expect("vacant").is_none());
        assert!(state.place(1, 0).expect("vacant").is_none());
        assert!(state.place(1, 1).expect("vacant").is_none());
        assert!(matches!(
            state.place(2, 0),
            Ok(Some(Winner::Player(Mark::X)))
        ));
    }
}
