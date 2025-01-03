mod app_window;
mod gst_backend;
mod projector_window;

use adw::Application;
use app_window::ShowtimeAppWindow;
use glib::{closure_local, Propagation};
use gst;
use gtk::prelude::*;
use gtk::{gio, glib};
use projector_window::ShowtimeProjectorWindow;

fn main() -> glib::ExitCode {
    gst::init().expect("Failed to init GStreamer!");
    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    gio::resources_register_include!("showtime.gresource").expect("Failed to include resources!");

    let application = Application::builder()
        .application_id("hu.doty.showtime")
        .build();

    application.connect_activate(|app| {
        let app_window = ShowtimeAppWindow::new(&app);

        let projector_window = ShowtimeProjectorWindow::new(app);
        projector_window.setup_player(app_window.sink());
        projector_window.present();

        app_window.connect_closure(
            "blackout",
            false,
            closure_local!(
                #[weak]
                projector_window,
                move |_win: ShowtimeAppWindow| {
                    projector_window.toggle_blackout();
                }
            ),
        );

        app_window.connect_close_request(move |_| {
            projector_window.close();
            Propagation::Proceed
        });

        app_window.present();
    });

    application.run()
}
