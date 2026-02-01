/**
 * Author: 
 * Date: 2026-02-01
 * Description: Implementierung der Spiel-Logik für das Tic-Tac-Toe-Spiel.
*/

use crate::ai::AIPlayer;
use crate::board::Board;
use crate::player::Player;

pub struct Game {
    board: Board,
    current_player: Player,
    human_player: Player,
    ai: AIPlayer,
    vs_ai: bool,
    game_over: bool,
}

impl Game {
    pub fn new(vs_ai: bool) -> Self {
        let human_player = Player::X;
        let ai_player = Player::O;

        Self {
            board: Board::new(),
            current_player: Player::X,
            human_player,
            ai: AIPlayer::new(ai_player),
            vs_ai,
            game_over: false,
        }
    }

    /// Führt einen Spielerzug auf dem gegebenen Feldindex aus.
    /// Wechselt den aktiven Spieler oder beendet das Spiel bei Gewinn/Unentschieden.
    pub fn make_move(&mut self, index: usize) -> bool {
        if self.game_over || !self.board.set(index, self.current_player) {
            return false;
        }

        if self.board.is_game_over() {
            self.game_over = true;
        } else {
            self.current_player = self.current_player.opponent();
        }

        true
    }

    /// Berechnet und führt den besten KI-Zug via Minimax aus.
    /// Gibt den gewählten Feldindex zurück oder None, falls kein Zug möglich ist.
    pub fn make_ai_move(&mut self) -> Option<usize> {
        if self.game_over || !self.vs_ai || self.current_player == self.human_player {
            return None;
        }

        if let Some(mov) = self.ai.get_best_move(&self.board) {
            self.board.set(mov, self.current_player);

            if self.board.is_game_over() {
                self.game_over = true;
            } else {
                self.current_player = self.current_player.opponent();
            }

            return Some(mov);
        }

        None
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.current_player = Player::X;
        self.game_over = false;
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.board.check_winner()
    }

    pub fn is_draw(&self) -> bool {
        self.board.is_full() && self.board.check_winner().is_none()
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn is_vs_ai(&self) -> bool {
        self.vs_ai
    }

    pub fn set_vs_ai(&mut self, vs_ai: bool) {
        self.vs_ai = vs_ai;
    }

    pub fn is_ai_turn(&self) -> bool {
        self.vs_ai && self.current_player != self.human_player && !self.game_over
    }
}
