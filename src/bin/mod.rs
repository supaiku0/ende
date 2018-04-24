extern crate gtk;
extern crate gdk;
extern crate ende;

use gtk::*;

#[macro_use]
mod utils;
mod app;
mod views;
mod error_view;
mod dialog;

use app::App;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let app = App::new();
    app.window.show_all();

    gtk::main();
}
