use gtk4 as gtk;

use gtk::glib::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use std::cell::Cell;

#[derive(Clone)]
pub struct AppWindow {
    window: ApplicationWindow,
    button: Button,
    state: Cell<bool>,
}

impl AppWindow {
    pub fn new(application: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(application)
            .title("Showtime")
            .default_width(350)
            .default_height(70)
            .build();
        return Self {
            window,
            button: Button::default(),
            state: Cell::new(false),
        };
    }

    pub fn init(&mut self) {
        self.button = Button::with_label("Click me!");
        self.button.connect_clicked(clone!(
            #[strong(rename_to = this)]
            self,
            move |btn| {
                this.button_clicked(btn);
            }
        ));
        self.window.set_child(Some(&self.button));
    }

    fn button_clicked(&self, button: &Button) {
        self.state.set(!self.state.get());
        if self.state.get() {
            button.set_label("Clicked!");
        } else {
            button.set_label("Click me!");
        }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
