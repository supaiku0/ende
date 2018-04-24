use super::gtk;
use super::gdk;

use gtk::*;
use std::rc::{Rc};
use std::sync::mpsc::{channel, Receiver};

use utils;
use dialog;
use ende::worker::{Worker, WorkerMode};

pub struct View {

    pub container: gtk::Box,
    window: Rc<gtk::Window>,
    drop_label: gtk::Label,
    worker: Worker,
    receiver: Receiver<String>

}

impl View {

    pub fn encryption_view(window: Rc<gtk::Window>) -> Rc<View> {
        View::new(window, WorkerMode::Encryption)
    }

    pub fn decryption_view(window: Rc<gtk::Window>) -> Rc<View> {
        View::new(window, WorkerMode::Decryption)
    }

    fn new(window: Rc<gtk::Window>, mode: WorkerMode) -> Rc<View> {

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let targets = utils::get_drop_targets();

        let description = match mode {
            WorkerMode::Encryption => "Drop files to encrypt",
            WorkerMode::Decryption => "Drop files to decrypt"
        };

        let drop_label = gtk::Label::new(description);
        drop_label.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
        container.pack_start(&drop_label, true, true, 0);

        let (tx, rx) = channel();

        let view = Rc::new(View {
            container,
            window,
            drop_label,
            worker: Worker::new(mode, tx),
            receiver: rx
        });

        {
            let drop_label = &view.drop_label;
            let view_clone = view.clone();
            drop_label.connect_drag_data_received(move |w, _, _, _, s, _, _| {

                let string = &s.get_text().expect("Couldn't get text");
                w.set_text(string);
            //    let x: () = view_clone.borrow_mut();
                view_clone.on_drop(string.to_owned());
            });
        }

        view
    }

    fn on_drop(&self, data: String) {

        let dialog = dialog::Dialog::new(&self.window);
        match dialog.run() {
            dialog::DialogResult::Ok => {},
            _ => return
        };

        self.worker.process(&data);

        // HACK: update the UI while waiting for
        // worker messages.
        loop {
            let received = self.receiver.try_recv();
            match received {

                Ok(ref data) => {
                    if data == "DONE" {
                        println!("?????");
                        break;
                    }

                    self.drop_label.set_text(&data);
                },

                Err(e) => {
                    println!("{:?}", e);
                    //break;
                }
            };

            gtk::main_iteration();
        }
    }

}
