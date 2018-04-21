extern crate gtk;
extern crate gdk;
extern crate ende;

use gtk::*;

mod app;
mod view;
mod utils;

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
