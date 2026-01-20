use crate::board::Board;
use crate::player::Player;

pub struct AIPlayer {
    player: Player,
}

impl AIPlayer {
    pub fn new(player: Player) -> Self {
        Self { player }
    }

    pub fn get_best_move(&self, board: &Board) -> Option<usize> {
        let mut best_score = i32::MIN;
        let mut best_move = None;

        for mov in board.available_moves() {
            let mut new_board = board.clone();
            new_board.set(mov, self.player);

            let score = self.minimax(&new_board, 0, i32::MIN, i32::MAX, false);

            if score > best_score {
                best_score = score;
                best_move = Some(mov);
            }
        }

        best_move
    }

    fn minimax(&self, board: &Board, depth: i32, mut alpha: i32, mut beta: i32, is_maximizing: bool) -> i32 {
        if let Some(winner) = board.check_winner() {
            return if winner == self.player {
                10 - depth
            } else {
                depth - 10
            };
        }

        if board.is_full() {
            return 0;
        }

        if is_maximizing {
            let mut max_eval = i32::MIN;
            for mov in board.available_moves() {
                let mut new_board = board.clone();
                new_board.set(mov, self.player);
                let eval = self.minimax(&new_board, depth + 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for mov in board.available_moves() {
                let mut new_board = board.clone();
                new_board.set(mov, self.player.opponent());
                let eval = self.minimax(&new_board, depth + 1, alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }
}
