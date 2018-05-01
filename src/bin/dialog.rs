extern crate gtk;

use gtk::*;
use utils;

#[derive(Clone)]
pub struct Dialog {
    dialog: gtk::MessageDialog,
    entry: gtk::Entry,
    entry_confirm: gtk::Entry,
    button_ok: gtk::Widget,
    button_cancel: gtk::Widget
}

enum DialogResult {
    Ok = -5,
    Cancel = -6
}

impl Dialog {

    pub fn new(parent: &gtk::Window) -> Self {

        let dialog = gtk::MessageDialog::new(
            Some(parent),
            gtk::DialogFlags::MODAL | gtk::DialogFlags::USE_HEADER_BAR,
            gtk::MessageType::Info,
            gtk::ButtonsType::None,
            "Choose a passphrase:"
        );

        let button_ok = dialog.add_button("Continue", -5);
        let button_cancel = dialog.add_button("Cancel", -6);
        button_ok.set_sensitive(false);
        utils::set_class(&button_ok, "suggested-action");

        let content_area = dialog.get_content_area();
        let entry = gtk::Entry::new();
        let entry_confirm = gtk::Entry::new();

        entry.set_visibility(false);
        entry.set_invisible_char('*');
        entry.set_size_request(250, 0);
        entry_confirm.set_visibility(false);
        entry_confirm.set_invisible_char('*');
        entry_confirm.set_size_request(250, 0);

        content_area.pack_end(&entry_confirm, false, false, 0);
        content_area.pack_end(&entry, false, false, 0);

        content_area.show_all();

        let dialog = Dialog { dialog, entry, entry_confirm, button_ok, button_cancel };
        dialog.entry.connect_key_release_event(clone!(dialog => move |_, _| {
            dialog.on_text_input();
            gtk::Inhibit(false)
        }));

        dialog.entry_confirm.connect_key_release_event(clone!(dialog => move |_, _| {
            dialog.on_text_input();
            gtk::Inhibit(false)
        }));

        dialog
    }

    pub fn run(&self) -> Option<String> {
        let dialog_result;
        match self.dialog.run() {
            -5 => dialog_result = DialogResult::Ok,
            _ => dialog_result = DialogResult::Cancel
        }

        let ret;
        if let DialogResult::Ok = dialog_result {
            ret = self.entry.get_text();
        } else {
            ret = None;
        }

        self.dialog.destroy();
        ret
    }

    fn on_text_input(&self) {
        let input1 = self.entry.get_text().expect("Failed to get text of entry.");
        let input2 = self.entry_confirm.get_text().expect("Failed to get text of entry.");
        //println!("{:?} == {:?}", input1, input2);
        self.button_ok.set_sensitive(input1.len() > 1 && input1 == input2);
    }

}
