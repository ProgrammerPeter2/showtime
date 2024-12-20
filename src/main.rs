mod app_window;
mod gst_backend;

use gtk4 as gtk;

use app_window::AppWindow;
use gst;
use gtk::prelude::*;
use gtk::{glib, Application};

fn main() -> glib::ExitCode {
    gst::init().expect("Failed to init GStreamer!");
    let application = Application::builder()
        .application_id("hu.peterhorvath.showtime")
        .build();

    application.connect_activate(|app| {
        let window = AppWindow::new(app);
        window.present();
    });

    application.run()
}
