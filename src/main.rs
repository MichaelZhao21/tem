use gtk::{glib, Application, ApplicationWindow, Button};
use gtk::prelude::*;

const APP_ID: &str = "xyz.michaelzhao.tem";

fn main() -> glib::ExitCode {
    // Create a new app
    let app = Application::builder().application_id(APP_ID).build();

    // Connect activate signal
    app.connect_activate(build_ui);

    // Run the app
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("tem")
        .child(&button)
        .build();

    // Present window
    window.present();
}
