use gtk4 as gtk;

use crate::gst_backend::GstBackend;
use async_channel::Sender;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Box, Button, Orientation, ProgressBar};
use std::cell::Cell;
use std::env;
use std::fmt::format;
use std::path::PathBuf;

#[derive(Clone)]
pub struct AppWindow {
    window: ApplicationWindow,
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

        let (sender, receiver) = async_channel::bounded::<u64>(1);
        let mut instance = Self {
            window,
            state: Cell::new(false),
            player: GstBackend::new(sender),
        };

        instance.init(receiver);
        instance
    }

    fn init(&self, receiver: async_channel::Receiver<u64>) {
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

        let progress_bar = ProgressBar::new();

        glib::spawn_future_local(clone!(
            #[weak]
            progress_bar,
            async move {
                while let Ok(position) = receiver.recv().await {
                    progress_bar.set_fraction(position as f64 / 157.0);
                }
            }
        ));

        let button = Button::with_label("Play");
        button.connect_clicked(clone!(
            #[strong(rename_to = this)]
            self,
            move |btn| {
                this.button_clicked(btn);
            }
        ));

        let gtk_box = Box::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .valign(Align::Center)
            .halign(Align::Center)
            .spacing(12)
            .orientation(Orientation::Vertical)
            .build();
        gtk_box.append(&progress_bar);
        gtk_box.append(&button);

        self.window.set_child(Some(&gtk_box));
    }

    fn button_clicked(&self, button: &Button) {
        self.state.set(!self.state.get());
        if self.state.get() {
            button.set_label("Pause");
        } else {
            button.set_label("Play");
        }
        self.player.toggle(self.state.get());
    }

    pub fn present(&self) {
        self.window.present();
    }
}
