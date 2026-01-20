mod player;
mod board;
mod ai;
mod game;
mod ui;

use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.example.TicTacToe";


fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(ui::build_ui);

    app.run()
}