mod app_window;
mod gst_backend;

use app_window::ShowtimeAppWindow;
use gst;
use gtk::prelude::*;
use gtk::{gio, glib, Application};

fn main() -> glib::ExitCode {
    gst::init().expect("Failed to init GStreamer!");
    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    gio::resources_register_include!("showtime.gresource").expect("Failed to include resources!");

    let application = Application::builder()
        .application_id("hu.peterhorvath.showtime")
        .build();

    application.connect_activate(|app| {
        let window = ShowtimeAppWindow::new(app);
        window.present();
    });

    application.run()
}
