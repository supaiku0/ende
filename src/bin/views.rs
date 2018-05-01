use super::gtk;
use super::gdk;

use gtk::*;
use std::rc::{Rc};
use std::sync::mpsc::{channel, Receiver};
use std::cell::Cell;

use utils;
use dialog;
use app;
use ende::worker::{Worker, WorkerMode};
use ende::archiver;

pub struct View {

    pub container: gtk::Box,
    app: Cell<Option<app::App>>,
    drop_label: gtk::Label,
    spinner: gtk::Spinner,
    info_label: gtk::Label,
    worker: Worker,
    receiver: Receiver<String>
}

impl View {

    pub fn encryption_view() -> Rc<View> {
        View::new(WorkerMode::Encryption)
    }

    pub fn decryption_view() -> Rc<View> {
        View::new(WorkerMode::Decryption)
    }

    fn new(mode: WorkerMode) -> Rc<View> {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Drop area
        let description = match mode {
            WorkerMode::Decryption => "Drop here to decrypt.",
            WorkerMode::Encryption => "Drop here to encrypt."
        };

        let drop_label = gtk::Label::new(description);
        let targets = utils::get_drop_targets();
        drop_label.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
        container.pack_start(&drop_label, true, true, 0);

        // Post drop area
        let spinner = gtk::Spinner::new();
        spinner.set_size_request(64, 64);
        container.pack_start(&spinner, false, false, 0);

        let info_label = gtk::Label::new("");
        container.pack_start(&info_label, false, false, 0);

        // View
        let (tx, rx) = channel();
        let view = Rc::new(View {
            container,
            app: Cell::new(None),
            drop_label,
            spinner,
            info_label,
            worker: Worker::new(mode, tx),
            receiver: rx
        });

        {
            let drop_label = &view.drop_label;
            let view_clone = view.clone();
            drop_label.connect_drag_data_received(move |sender, _, _, _, data, _, _| {

                let string = &data.get_text().expect("Couldn't get text");
                sender.set_text(string);
                view_clone.on_drop(string.to_owned());
            });
        }

        view
    }

    pub fn on_show(&self, app: app::App) {
        self.app.set(Some(app));
        self.enter_drop_mode();
    }

    fn on_drop(&self, data: String) {
        let paths = archiver::get_paths(&data);
        if paths.is_empty() {
            self.drop_label.set_text(format!("Failed to parse paths from input:\n{:?}", data).as_str());
            return;
        }

        let app = self.app.take();
        let dialog = dialog::Dialog::new(&app.clone().unwrap().window);
        self.app.set(app);

        if let Some(passphrase) = dialog.run() {
            assert!(!passphrase.is_empty());
            self.enter_work_mode();
            self.worker.process(paths, passphrase);

        } else {
            return;
        }

        // HACK: update the UI while waiting for
        // worker messages.
        loop {
            let received = self.receiver.try_recv();
            match received {

                Ok(ref data) => {
                    if data == "DONE" {
                        //println!("?????");
                        self.info_label.set_text("Completed.");
                        break;
                    } else {
                        self.info_label.set_text(&data);
                    }

                },

                Err(e) => {
                    //println!("{:?}", e);
                    self.info_label.set_text(&format!("{:?}", e));
                }
            };

            gtk::main_iteration();
        }

        self.spinner.hide();

        let app = self.app.take();
        app.clone().unwrap().show_continue();
        self.app.set(app);
    }

    pub fn enter_work_mode(&self) {
        let app = self.app.take();
        app.clone().unwrap().enter_work_mode();
        self.app.set(app);

        self.drop_label.hide();
        self.spinner.show();
        self.spinner.start();
        self.info_label.show();
        self.container.set_hexpand(false);
        self.container.set_vexpand(false);
        self.container.set_valign(gtk::Align::Center);
        self.container.set_halign(gtk::Align::Center);
    }

    pub fn enter_drop_mode(&self) {
        let app = self.app.take();
        app.clone().unwrap().enter_drop_mode();
        self.app.set(app);

        self.info_label.set_text("");

        self.spinner.stop();
        self.spinner.hide();
        self.info_label.hide();
        self.drop_label.show();
        self.container.set_hexpand(true);
        self.container.set_vexpand(true);
        self.container.set_valign(gtk::Align::Fill);
        self.container.set_halign(gtk::Align::Fill);
    }

}
