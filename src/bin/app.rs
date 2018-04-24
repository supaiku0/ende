use super::gtk;

use ende;

use std::rc::Rc;

use gtk::*;
use utils::set_class;
use views::View;
use error_view::ErrorView;

pub struct App {

    pub window: Rc<Window>,
    header: HeaderBar,
    stack_switcher: StackSwitcher

}

impl App {

    pub fn new() -> Self {
        let header = HeaderBar::new();
        header.set_show_close_button(true);
        header.set_title("Ende");
        header.set_subtitle("Easy drag & drop en-/decryption");

        let window = Rc::new(Window::new(WindowType::Toplevel));
        window.set_default_size(540, 360);
        window.set_titlebar(&header);

        let settings = MenuButton::new();
        settings.set_image(&Image::new_from_icon_name("preferences-system", 0));
        //settings.set_popover(&popover);
        settings.set_use_popover(true);

        header.pack_end(&settings);

        let stack = Stack::new();

        let stack_switcher = StackSwitcher::new();
        stack_switcher.set_stack(&stack);
        header.pack_start(&stack_switcher);

        let encrypt_page = Paned::new(Orientation::Horizontal);
        let decrypt_page = Paned::new(Orientation::Horizontal);

        stack.add_titled(&encrypt_page, "encrypt", "Encrypt");
        stack.add_titled(&decrypt_page, "decrypt", "Decrypt");

        let encryption_view = View::encryption_view(window.clone());
        let decryption_view = View::decryption_view(window.clone());

        encrypt_page.pack1(&encryption_view.container, true, true);
        decrypt_page.pack1(&decryption_view.container, true, true);

        let result = ende::check_required_binaries();
        if result.is_err() {

            let error_page = Paned::new(Orientation::Horizontal);
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

        window.connect_show(clone!(stack_switcher => move |_| {
            if result.is_err() {
                stack_switcher.hide();
            }
        }));

        App {
            window,
            header,
            stack_switcher
        }
    }
}
