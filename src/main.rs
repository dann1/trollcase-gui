extern crate gtk;
use gtk::{prelude::*};

mod casing;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Window properties
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("TrollCase");
    window.set_position(gtk::WindowPosition::Center);

    let context = gtk::StyleContext::new();
    context.add_class("system-font");

    let grid = gtk::Grid::new();
    let label = gtk::Label::new(Some("Enter text:"));
    let entry = gtk::Entry::new();

    // Setup buttons

    let btn_randomize = gtk::Button::with_label("Randomize");
    let btn_alternate = gtk::Button::with_label("Alternate");

    btn_alternate.set_halign(gtk::Align::End);
    grid.attach(&label, 0, 0, 1, 1);
    grid.attach(&entry, 1, 0, 2, 1);
    grid.attach(&btn_randomize, 0, 1, 1, 1);
    grid.attach(&btn_alternate, 2, 1, 1, 1);

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

    // Window management

    window.show_all();
    window.present();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        glib::Propagation::Stop
    });

    gtk::main();
}
