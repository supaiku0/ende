use super::gtk;

use ende;

use std::rc::Rc;

use gtk::*;
use views::View;
use error_view::ErrorView;

#[derive(Clone)]
pub struct App {

    pub window: gtk::Window,
    header: gtk::HeaderBar,
    stack_switcher: gtk::StackSwitcher,
    settings: gtk::MenuButton,
    continue_button: gtk::Button,
    encryption_view: Rc<View>,
    decryption_view: Rc<View>

}

impl App {

    pub fn new() -> Self {
        let header = gtk::HeaderBar::new();
        header.set_show_close_button(true);
        header.set_title("Ende");
        header.set_subtitle("Easy drag & drop en-/decryption");

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_default_size(540, 360);
        window.set_titlebar(&header);

        let settings = gtk::MenuButton::new();
        settings.set_image(&gtk::Image::new_from_icon_name("preferences-system", 0));
        //settings.set_popover(&popover);
        settings.set_use_popover(true);

        let continue_button = gtk::Button::new_with_label("Continue");

        header.pack_end(&settings);
        header.pack_end(&continue_button);

        let stack = gtk::Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);

        let stack_switcher = gtk::StackSwitcher::new();
        stack_switcher.set_stack(&stack);
        header.pack_start(&stack_switcher);

        let encrypt_page = gtk::Paned::new(gtk::Orientation::Horizontal);
        let decrypt_page = gtk::Paned::new(gtk::Orientation::Horizontal);

        stack.add_titled(&encrypt_page, "encrypt", "Encrypt");
        stack.add_titled(&decrypt_page, "decrypt", "Decrypt");

        let encryption_view = View::encryption_view();
        let decryption_view = View::decryption_view();

        encrypt_page.pack1(&encryption_view.container, true, true);
        decrypt_page.pack1(&decryption_view.container, true, true);

        let result = ende::check_required_binaries();
        if result.is_err() {

            let error_page = gtk::Paned::new(gtk::Orientation::Horizontal);
            let error_view = ErrorView::new(result.clone().unwrap_err());
            error_page.pack1(&error_view.container, true, true);
            error_page.show();

            stack.add_named(&error_page, "error");

        } else {
            stack_switcher.show_all();
        }

        window.add(&stack);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let app = App {
            window,
            header,
            stack_switcher,
            settings,
            continue_button,
            encryption_view,
            decryption_view
        };

        app.window.connect_show(clone!(app => move |_| {
            app.on_show(result.is_err());
        }));

        app.continue_button.connect_clicked(clone!(app => move |_| {
            app.encryption_view.enter_drop_mode();
            app.decryption_view.enter_drop_mode();

            app.enter_drop_mode();
        }));

        app
    }

    pub fn enter_drop_mode(&self) {
        self.stack_switcher.set_sensitive(true);
        self.settings.set_sensitive(true);
        self.continue_button.hide();
    }

    pub fn enter_work_mode(&self) {
        self.stack_switcher.set_sensitive(false);
        self.settings.set_sensitive(false);
    }

    pub fn show_continue(&self) {
        self.continue_button.show();
    }

    pub fn on_show(&self, missing_deps: bool) {
        if missing_deps {
            self.stack_switcher.hide();
            self.continue_button.hide();
        }

        self.encryption_view.on_show(self.clone());
        self.decryption_view.on_show(self.clone());
    }
}
