extern crate gtk;
use glib::Propagation;
use gtk::{prelude::*, Button, Entry, Grid, Label, Window, WindowType};

mod casing;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Create window properties
    let window = Window::new(WindowType::Toplevel);

    window.set_title("TrollCase");
    window.set_position(gtk::WindowPosition::Center);
    let grid = Grid::new();
    let label = Label::new(Some("Enter text:"));
    let entry = Entry::new();

    // Setup buttons

    let mut btn_name = "Randomize".to_string();
    casing::randomize(&mut btn_name);

    let btn_randomize = Button::with_label(&btn_name);
    let btn_alternate = Button::with_label("AlTeRnAtE");

    grid.attach(&label, 0, 0, 1, 1);
    grid.attach(&entry, 1, 0, 2, 1);
    grid.attach(&btn_randomize, 0, 1, 1, 1);
    grid.attach(&btn_alternate, 1, 1, 1, 1);

    window.add(&grid);

    // Functionality

    fn connect_button_with_caser<F>(button: &gtk::Button, entry: &gtk::Entry, caser: F)
    where
        F: Fn(&mut String) -> String + 'static,
    {
        button.connect_clicked(glib::clone!(@weak entry => move |_| {
            let mut text = entry.text().to_string();
            let result = caser(&mut text);
            entry.set_text(&result);
        }));
    }

    connect_button_with_caser(&btn_randomize, &entry, casing::randomize);
    connect_button_with_caser(&btn_alternate, &entry, casing::alternate);

    window.show_all();
    window.present();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Propagation::Stop
    });

    gtk::main();
}
