use super::gtk;
use gtk::*;

pub struct ErrorView {
    pub container: gtk::Box
}

impl ErrorView {

    pub fn new(messages: Vec<&'static str>) -> Self {

        let error_message = "The following programs are required, but could not be found: \n";
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.set_hexpand(false);
        container.set_vexpand(false);
        container.set_valign(gtk::Align::Center);
        container.set_halign(gtk::Align::Center);

        let mut label = gtk::Label::new(error_message.to_owned().as_str());
        container.pack_start(&label, false, false, 0);

        for message in messages {
            label = gtk::Label::new(message);
            container.pack_start(&label, false, false, 0);
        }

        ErrorView { container }
    }

}
