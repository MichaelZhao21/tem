mod window;

use gtk::{glib, gio, Application};
use window::Window;
use gtk::prelude::*;

const APP_ID: &str = "xyz.michaelzhao.tem";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources.");
    
    // Create a new app
    let app = Application::builder().application_id(APP_ID).build();

    // Connect activate signal
    app.connect_activate(build_ui);

    // Run the app
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and present it
    let window = Window::new(app);
    window.present();
}
