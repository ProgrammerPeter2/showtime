mod app_window;

use gtk4 as gtk;

use app_window::AppWindow;
use gtk::prelude::*;
use gtk::{glib, Application};

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("hu.peterhorvath.showtime")
        .build();

    application.connect_activate(|app| {
        let mut window = AppWindow::new(app);
        window.init();
        window.present();
    });

    application.run()
}
