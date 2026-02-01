Tic-Tac-Toe in Rust

## Überblick

**TicTacHoe** ist eine vollständig implementierte Tic-Tac-Toe-Anwendung in Rust mit einer graphischen Benutzeroberfläche (GTK4). Das Spiel unterstützt sowohl Spieler-gegen-Spieler als auch Spieler-gegen-KI Modi mit einem intelligenten Minimax-Algorithmus.

## Features

- ✅ **Graphische Benutzeroberfläche (GTK4)** - Moderne und intuitive UI
- ✅ **KI-Gegner** - Minimax-Algorithmus mit Alpha-Beta-Pruning für optimales Spiel
- ✅ **Scoreboard** - Verfolgung von Siegen und Unentschieden
- ✅ **Spielmodus-Umschaltung** - Wechsel zwischen vs Computer und vs Spieler
- ✅ **Spielstatus-Anzeige** - Aktuelle Spielinformationen in Echtzeit

## Projektstruktur

```
TicTacHoe/
├── src/
│   ├── main.rs      # Haupteinstiegspunkt der Anwendung
│   ├── game.rs      # Spiel-Logik und Spielverwaltung
│   ├── board.rs     # Spielbrett-Implementation
│   ├── player.rs    # Spieler-Definitionen (X und O)
│   ├── ai.rs        # KI-Logik mit Minimax-Algorithmus
│   └── ui.rs        # GTK4 Benutzeroberfläche
├── Cargo.toml       # Projekt-Abhängigkeiten
└── README.md        # Diese Datei
```

## Module

### `main.rs`
Der Einstiegspunkt der Anwendung. Initialisiert die GTK4-Anwendung und startet die UI.

```rust
const APP_ID: &str = "org.example.TicTacToe";
```

### `player.rs`
Definiert die Spieler als Enum mit Symbolen und Opponent-Logik.

```rust
pub enum Player {
    X,
    O,
}
```

**Methoden:**
- `symbol()` - Gibt "X" oder "O" zurück
- `opponent()` - Bestimmt den gegnerischen Spieler

### `board.rs`
Verwaltet das 3x3 Spielfeld mit 9 Zellen.

**Wichtige Methoden:**
- `new()` - Erstellt ein leeres Spielfeld
- `set(index, player)` - Setzt einen Spieler auf ein Feld
- `check_winner()` - Prüft auf Gewinner (8 Gewinnmuster)
- `available_moves()` - Gibt Liste verfügbarer Züge zurück
- `is_game_over()` - Prüft ob Spiel beendet ist

### `game.rs`
Zentrale Spiel-Logik, verwaltet Spielfluss und Zustandsverwaltung.

**Wichtige Methoden:**
- `new(vs_ai)` - Erstellt neues Spiel (mit/ohne KI)
- `make_move(index)` - Führt einen Zug aus
- `ai_move()` - Lässt KI ihren Zug berechnen und ausführen
- `reset()` - Setzt das Spiel zurück
- `current_player()` - Gibt aktuellen Spieler zurück
- `get_winner()` - Gibt Gewinner zurück, falls vorhanden
- `is_draw()` - Prüft auf Unentschieden

### `ai.rs`
Implementiert die Künstliche Intelligenz mit Minimax-Algorithmus.

**Algorithmus: Minimax mit Alpha-Beta-Pruning**
- **Bewertung:** 
  - `+10` = KI gewinnt
  - `-10` = Gegner gewinnt
  - `0` = Unentschieden
- **Optimierung:** Alpha-Beta-Pruning für schnellere Berechnungen

```rust
pub fn get_best_move(&self, board: &Board) -> Option<usize>
```

### `ui.rs`
GTK4-basierte Benutzeroberfläche mit Event-Handling.

**Komponenten:**
- **Spielfeld:** 3x3 Grid mit Buttons für jeden Zug
- **Scoreboard:** Anzeige von X-Siegen, O-Siegen und Unentschieden
- **Status-Label:** Zeigt aktuellen Spieler oder Spielende
- **Mode-Button:** Umschalten zwischen KI und Spieler-Modus
- **Neue Spiel-Buttons:** Spiel neustarten

## Abhängigkeiten

```toml
[dependencies]
gtk = { version = "0.10.3", package = "gtk4", features = ["v4_20"] }
```

- **GTK4**: Für die graphische Benutzeroberfläche

## Installation und Ausführung

### Anforderungen
- Rust 1.70+ (Edition 2024)
- GTK4 Entwicklungsbibliotheken
- Linux/macOS/Windows

### Kompilierung

```bash
cargo build --release
```

### Ausführung

```bash
cargo run
```

## Gewinnmuster

Das Spiel prüft folgende 8 Muster:

```
Reihen:        Spalten:       Diagonalen:
[0] [1] [2]    [0] [3] [6]    [0] [4] [8]
[3] [4] [5]    [1] [4] [7]    [2] [4] [6]
[6] [7] [8]    [2] [5] [8]
```

## KI-Strategie

Die KI nutzt den **Minimax-Algorithmus mit Alpha-Beta-Pruning**:

1. **Evaluierung:** Alle möglichen Spielzüge werden bewertet
2. **Minimax:** Maximiert KI-Gewinn, minimiert Gegner-Gewinn
3. **Alpha-Beta-Pruning:** Eliminiert unnötige Rechenzweige
4. **Resultat:** Optimaler Zug mit hoher Spielstärke

Die KI kann nie verlieren und spielt immer optimal.

## Scoreboard

Das Scoreboard verfolgt:
- **X-Siege:** Gewonnen von Spieler X
- **O-Siege:** Gewonnen von Spieler O (oder KI)
- **Unentschieden:** Anzahl der Remis

Werte bleiben über mehrere Spiele erhalten.



