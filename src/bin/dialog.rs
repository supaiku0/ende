extern crate gtk;

use gtk::{DialogExt, WidgetExt};

pub struct Dialog {
    dialog: gtk::MessageDialog
}

pub enum DialogResult {
    Ok = -5,
    Cancel = -6
}

impl Dialog {

    pub fn new(parent: &gtk::Window) -> Self {

        let dialog = gtk::MessageDialog::new(
            Some(parent),
            gtk::DialogFlags::MODAL,
            gtk::MessageType::Info,
            gtk::ButtonsType::OkCancel,
            ".."
        );

        Dialog { dialog }
    }

    pub fn run(&self) -> DialogResult {
        let status = self.dialog.run();
        self.dialog.destroy();

        match status {
            -5 => return DialogResult::Ok,
            _ => return DialogResult::Cancel
        }
    }

}
