use gtk4 as gtk;

use crate::gst_backend::GstBackend;
use gtk::glib::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use std::cell::Cell;
use std::env;
use std::fmt::format;
use std::path::PathBuf;

#[derive(Clone)]
pub struct AppWindow {
    window: ApplicationWindow,
    button: Button,
    state: Cell<bool>,
    player: GstBackend,
}

impl AppWindow {
    pub fn new(application: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(application)
            .title("Showtime")
            .default_width(350)
            .default_height(70)
            .build();

        let mut instance = Self {
            window,
            button: Button::default(),
            state: Cell::new(false),
            player: GstBackend::new(),
        };

        instance.init();
        instance
    }

    fn init(&mut self) {
        match env::current_dir() {
            Ok(path) => {
                let path: String = format!(
                    "file://{}",
                    path.with_file_name("song.mp3")
                        .to_str()
                        .expect("Failed to parse path!")
                );
                self.player.set_song_uri(Some(path.as_str()));
            }
            Err(err) => eprintln!("{:?}", err),
        }
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
        self.player.toggle(self.state.get());
    }

    pub fn present(&self) {
        self.window.present();
    }
}
