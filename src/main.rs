use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gtk::init()?;

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let file = std::fs::File::create("mylog.txt")?;
    let file = RefCell::new(file);

    button.connect_clicked(move |_| {
        use std::io::Write;
        match file.borrow_mut().write_all(b"I was clicked") {
            Ok(_) => (),
            Err(e) => eprintln!("Error writing to file: {}", e),
        }
    });

    gtk::main();

    Ok(())
}
