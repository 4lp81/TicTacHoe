use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Grid, Label, Box, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

use crate::game::Game;
use crate::player::Player;

/// Baut die gesamte GTK4-Oberfläche auf: Spielfeld, Scoreboard,
/// Steuerungsbuttons und verknüpft alle Event-Handler.
pub fn build_ui(app: &Application) {
    let game = Rc::new(RefCell::new(Game::new(true)));

    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    let status_label = Label::new(Some("Spieler X ist dran"));
    status_label.add_css_class("status-label");

    let scoreboard: Rc<RefCell<(u32,u32,u32)>> = Rc::new(RefCell::new((0, 0,0)));
    let x_score_label = Label::new(Some("X: 0"));
    x_score_label.add_css_class("score-label");
    let o_score_label = Label::new(Some("O: 0"));
    o_score_label.add_css_class("score-label");
    let draw_score_label = Label::new(Some("Unentschieden: 0"));
    draw_score_label.add_css_class("score-label");

    let score_box = Box::new(Orientation::Horizontal, 10);
    score_box.append(&x_score_label);
    score_box.append(&o_score_label);
    score_box.append(&draw_score_label);


    let mode_button = Button::with_label("Modus: vs Computer");
    mode_button.add_css_class("mode-button");

    let grid = Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);
    grid.set_halign(gtk::Align::Center);

    let buttons: Rc<RefCell<Vec<Button>>> = Rc::new(RefCell::new(Vec::new()));

    for i in 0..9 {
        let button = Button::with_label("");
        button.set_size_request(80, 80);
        button.add_css_class("game-button");

        let row = (i / 3) as i32;
        let col = (i % 3) as i32;
        grid.attach(&button, col, row, 1, 1);

        let game_clone = Rc::clone(&game);
        let buttons_clone = Rc::clone(&buttons);
        let status_label_clone = status_label.clone();
        let scoreboard_clone = Rc::clone(&scoreboard);
        let o_score_label_clone = o_score_label.clone();
        let x_score_label_clone = x_score_label.clone();
        let draw_score_label_clone = draw_score_label.clone();

        button.connect_clicked(move |btn| {
            let mut g = game_clone.borrow_mut();

            let current = g.current_player();
            if !g.make_move(i) {
                return;
            }

            btn.set_label(current.symbol());
            match current {
                Player::X => btn.add_css_class("player-x"),
                Player::O => btn.add_css_class("player-o"),
            }

            if let Some(winner) = g.get_winner() {
                match winner {
                    Player::X => {
                        scoreboard_clone.borrow_mut().0 += 1;
                        x_score_label_clone.set_label(&format!("X: {}", scoreboard_clone.borrow().0));
                    }
                    Player::O => {
                        scoreboard_clone.borrow_mut().1 += 1;
                        o_score_label_clone.set_text(&format!("O: {}", scoreboard_clone.borrow().1));
                    }
                }
            } else if g.is_draw(){
                scoreboard_clone.borrow_mut().2 += 1;
                draw_score_label_clone.set_text(&format!("Unentschieden: {}", scoreboard_clone.borrow().2));
            }

            update_status(&status_label_clone, &g);

            if g.is_ai_turn() {
                drop(g);

                let game_for_ai = Rc::clone(&game_clone);
                let buttons_for_ai = Rc::clone(&buttons_clone);
                let status_for_ai = status_label_clone.clone();
                let scoreboard_for_ai = Rc::clone(&scoreboard_clone);
                let o_score_label_for_ai = o_score_label_clone.clone();
                let x_score_label_for_ai = x_score_label_clone.clone();
                let draw_score_label_for_ai = draw_score_label_clone.clone();

                glib::timeout_add_local_once(std::time::Duration::from_millis(300), move || {
                    let mut g = game_for_ai.borrow_mut();
                    if let Some(ai_move) = g.make_ai_move() {
                        let btns = buttons_for_ai.borrow();
                        btns[ai_move].set_label(Player::O.symbol());
                        btns[ai_move].add_css_class("player-o");

                        if let Some(winner) = g.get_winner() {
                            match winner {
                                Player::X => {
                                    scoreboard_for_ai.borrow_mut().0 += 1;
                                    x_score_label_for_ai.set_label(&format!("X: {}", scoreboard_for_ai.borrow().0));
                                }
                                Player::O => {
                                    scoreboard_for_ai.borrow_mut().1 += 1;
                                    o_score_label_for_ai.set_text(&format!("O: {}", scoreboard_for_ai.borrow().1));
                                }
                            }
                        } else if g.is_draw(){
                            scoreboard_for_ai.borrow_mut().2 += 1;
                            draw_score_label_for_ai.set_text(&format!("Unentschieden: {}", scoreboard_for_ai.borrow().2));
                        }

                        update_status(&status_for_ai, &g);
                    }
                });
            }
        });

        buttons.borrow_mut().push(button);
    }

    let reset_button = Button::with_label("Neues Spiel");
    reset_button.add_css_class("reset-button");

    let game_for_reset = Rc::clone(&game);
    let buttons_for_reset = Rc::clone(&buttons);
    let status_for_reset = status_label.clone();

    reset_button.connect_clicked(move |_| {
        game_for_reset.borrow_mut().reset();
        status_for_reset.set_label("Spieler X ist dran");

        for btn in buttons_for_reset.borrow().iter() {
            btn.set_label("");
            btn.remove_css_class("player-x");
            btn.remove_css_class("player-o");
        }
    });

    let score_reset_button = Button::with_label("Scoreboard Reset");
    score_reset_button.add_css_class("reset-button");

    let scoreboard_for_score_reset = Rc::clone(&scoreboard);
    let x_score_label_for_reset = x_score_label.clone();
    let o_score_label_for_reset = o_score_label.clone();
    let draw_score_label_for_reset = draw_score_label.clone();

    score_reset_button.connect_clicked(move |_| {
        *scoreboard_for_score_reset.borrow_mut() = (0, 0, 0);
        x_score_label_for_reset.set_label("X: 0");
        o_score_label_for_reset.set_label("O: 0");
        draw_score_label_for_reset.set_label("Unentschieden: 0");
    });

    let game_for_mode = Rc::clone(&game);
    let buttons_for_mode = Rc::clone(&buttons);
    let status_for_mode = status_label.clone();

    mode_button.connect_clicked(move |btn| {
        let mut g = game_for_mode.borrow_mut();
        let new_vs_ai = !g.is_vs_ai();
        g.set_vs_ai(new_vs_ai);
        g.reset();

        btn.set_label(if new_vs_ai {
            "Modus: vs Computer"
        } else {
            "Modus: 2 Spieler"
        });

        status_for_mode.set_label("Spieler X ist dran");

        for b in buttons_for_mode.borrow().iter() {
            b.set_label("");
            b.remove_css_class("player-x");
            b.remove_css_class("player-o");
        }
    });

    main_box.append(&status_label);
    main_box.append(&mode_button);
    main_box.append(&score_box);
    main_box.append(&grid);
    main_box.append(&reset_button);
    main_box.append(&score_reset_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tic Tac Toe")
        .default_width(320)
        .default_height(450)
        .child(&main_box)
        .build();

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_string(
        r#"
        .status-label {
            font-size: 18px;
            font-weight: bold;
            margin-bottom: 10px;
        }
        .game-button {
            font-size: 32px;
            font-weight: bold;
            min-width: 80px;
            min-height: 80px;
        }
        .player-x {
            color: #e74c3c;
        }
        .player-o {
            color: #3498db;
        }
        .reset-button {
            margin-top: 15px;
            padding: 10px 20px;
        }
        .score-label {
            font-size: 14px;
            font-weight: bold;
        }
        .mode-button {
            margin-bottom: 10px;
        }
        "#,
    );

    gtk::style_context_add_provider_for_display(
        &gtk::prelude::WidgetExt::display(&window),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.present();
}

/// Aktualisiert das Status-Label basierend auf dem aktuellen Spielzustand
/// (Gewinner, Unentschieden oder nächster Spieler).
fn update_status(label: &Label, game: &Game) {
    if let Some(winner) = game.get_winner() {
        let msg = if game.is_vs_ai() && winner == Player::O {
            "Computer hat gewonnen!".to_string()
        } else {
            format!("Spieler {} hat gewonnen!", winner.symbol())
        };
        label.set_label(&msg);
    } else if game.is_draw() {
        label.set_label("Unentschieden!");
    } else {
        let msg = if game.is_vs_ai() && game.current_player() == Player::O {
            "Computer denkt...".to_string()
        } else {
            format!("Spieler {} ist dran", game.current_player().symbol())
        };
        label.set_label(&msg);
    }
}
