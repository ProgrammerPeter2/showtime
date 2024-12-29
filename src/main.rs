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
        use gstgtk4::RenderWidget;
        use gtk::Window;

        let app_window = ShowtimeAppWindow::new(&app);

        let projector_window = Window::builder()
            .application(app)
            .title("Showtime presenter")
            .build();
        let render_widget = RenderWidget::new(app_window.sink());
        render_widget.set_size_request(1250, 720);
        projector_window.set_child(Some(&render_widget));
        projector_window.present();
        app_window.present();
    });

    application.run()
}
