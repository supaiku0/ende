use super::gtk;

use gtk::*;
use utils::set_class;
use view::View;

pub struct App {

    pub window: Window,
    header: HeaderBar,
    stack_switcher: StackSwitcher

}

impl App {

    pub fn new() -> App {
        let header = HeaderBar::new();
        header.set_show_close_button(true);
        header.set_title("Ende");
        header.set_subtitle("Easy drag & drop en-/decryption");

        let window = Window::new(WindowType::Toplevel);
        window.set_default_size(540, 360);
        window.set_titlebar(&header);

        let button = Button::new_with_label("Start");
        set_class(&button, "suggested-action");

        button.connect_clicked(|_| {
            println!("Clicked!");
        });


        let settings = MenuButton::new();
        settings.set_image(&Image::new_from_icon_name("preferences-system", 0));
        //settings.set_popover(&popover);
        settings.set_use_popover(true);

        header.pack_end(&settings);
        header.pack_end(&button);

        let stack = Stack::new();
        let stack_switcher = StackSwitcher::new();
        stack_switcher.set_stack(&stack);
        header.pack_start(&stack_switcher);

        let encrypt_page = Paned::new(Orientation::Horizontal);
        let decrypt_page = Paned::new(Orientation::Horizontal);

        stack.add_titled(&encrypt_page, "encrypt", "Encrypt");
        stack.add_titled(&decrypt_page, "decrypt", "Decrypt");
        stack.set_visible_child_full("encrypt", StackTransitionType::None);

        let encryption_view = View::encryption_view();
        let decryption_view = View::decryption_view();

        encrypt_page.pack1(&encryption_view.container, true, true);
        decrypt_page.pack1(&decryption_view.container, true, true);

        window.add(&stack);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            stack_switcher
        }
    }
}
