use fltk::{app, button::Button, input::Input, prelude::*, window::Window};
mod casing;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    // Window properties
    let window = Window::new(100, 100, 400, 150, "TrollCase");

    // Setup text input
    let mut input = Input::new(60, 20, 270, 25, "");

    // Setup buttons
    let mut btn_randomize = Button::new(50, 100, 80, 40, "Randomize");
    let mut btn_alternate = Button::new(270, 100, 80, 40, "Alternate");

    // Functionality
    fn connect_button_with_caser<F>(button: &mut Button, input: &mut Input, caser: F)
    where
        F: Fn(&str) -> String + 'static,
    {
        let mut input = input.clone();
        button.set_callback(move |_| {
            let text = input.value();
            let result = caser(&text);
            input.set_value(&result);
        });
    }

    connect_button_with_caser(&mut btn_randomize, &mut input, casing::randomize);
    connect_button_with_caser(&mut btn_alternate, &mut input, casing::alternate);

    // TODO: Copy to clipboard button

    window.center_screen().show();
    app.run().unwrap();
}
