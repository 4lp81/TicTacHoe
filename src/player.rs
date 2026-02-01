/**
 * Author: 
 * Date: 2026-02-01
 * Description: Definition der Spieler für das Tic-Tac-Toe-Spiel.
*/

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn symbol(&self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }

    pub fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
