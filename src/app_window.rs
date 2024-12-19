use gtk4 as gtk;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

pub struct AppWindow {
    window: ApplicationWindow,
}

impl AppWindow {
    pub fn new(application: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(application)
            .title("Showtime")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));
        return Self { window: window };
    }

    pub fn present(&self) {
        self.window.present();
    }
}
