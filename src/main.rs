use fltk::{app, button::Button, input::Input, prelude::*, window::Window};
mod casing;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    // Window properties
    let window = Window::new(100, 100, 400, 150, button_title("trollcase", casing::randomize));

    // Setup text input
    let input = Input::new(60, 20, 270, 25, "");

    // Create buttons
    let mut btn_randomize = Button::new(50, 100, 90, 40, button_title("randomize", casing::randomize));
    let mut btn_alternate = Button::new(270, 100, 90, 40, button_title("alternate", casing::alternate));
    let mut btn_copy = Button::new(350, 23, 35, 20, "copy");

    // Casers
    connect_button_with_caser(&mut btn_randomize, &input, casing::randomize);
    connect_button_with_caser(&mut btn_alternate, &input, casing::alternate);

    // Clipboard
    btn_copy.set_callback({
        let input = input.clone();
        move |_| {
            app::copy(&input.value())
        }
    });

    window.center_screen().show();
    app.run().unwrap();
}

fn button_title<F>(title: &str, caser: F) -> &'static str
where
    F: Fn(&str) -> String,
{
    return Box::leak(caser(title).into_boxed_str());
}


fn connect_button_with_caser<F>(button: &mut Button, input: &Input, caser: F)
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

