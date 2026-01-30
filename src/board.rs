use crate::player::Player;

#[derive(Clone)]
pub struct Board {
    cells: [Option<Player>; 9],
}

impl Board {
    pub fn new() -> Self {
        Self { cells: [None; 9] }
    }

    /// Setzt einen Spieler auf das Feld am gegebenen Index.
    /// Gibt false zurück, falls das Feld bereits belegt ist.
    pub fn set(&mut self, index: usize, player: Player) -> bool {
        if self.cells[index].is_none() {
            self.cells[index] = Some(player);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.cells = [None; 9];
    }

    pub fn available_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| if cell.is_none() { Some(i) } else { None })
            .collect()
    }

    /// Prüft alle 8 Gewinnmuster (3 Reihen, 3 Spalten, 2 Diagonalen)
    /// und gibt den Gewinner zurück, falls vorhanden.
    pub fn check_winner(&self) -> Option<Player> {
        const WIN_PATTERNS: [[usize; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Reihen
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Spalten
            [0, 4, 8], [2, 4, 6],             // Diagonalen
        ];

        for pattern in WIN_PATTERNS {
            if let Some(player) = self.cells[pattern[0]] {
                if self.cells[pattern[1]] == Some(player) && self.cells[pattern[2]] == Some(player) {
                    return Some(player);
                }
            }
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_some())
    }

    pub fn is_game_over(&self) -> bool {
        self.check_winner().is_some() || self.is_full()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
