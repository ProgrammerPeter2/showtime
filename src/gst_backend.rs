use async_channel::Sender;
use glib::clone;
use gst::{prelude::ObjectExt, Element};
use gst_play::{Play, PlaySignalAdapter};
use gtk::glib;

#[derive(Debug, Clone)]
pub struct GstBackend {
    sender: Sender<u64>,
    play: Play,
    signals: PlaySignalAdapter,
    gtksink: Element,
}

impl GstBackend {
    pub fn new(sender: Sender<u64>) -> Self {
        let play = Play::default();
        let gtksink = gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .unwrap();

        play.pipeline().set_property("video-sink", &gtksink);

        let signals = PlaySignalAdapter::new(&play);
        let instance = Self {
            sender,
            play,
            signals,
            gtksink,
        };
        instance.setup_signals();
        instance
    }

    fn setup_signals(&self) {
        self.signals.connect_position_updated(clone!(
            #[strong(rename_to = sender)]
            self.sender,
            move |_, pos| {
                sender.send_blocking(pos.unwrap().seconds()).unwrap();
            }
        ));
    }
    pub fn set_song_uri(&self, uri: Option<&str>) {
        if uri.is_some() {
            self.play.set_uri(uri);
        }
    }

    pub fn play(&self) {
        self.play.play();
    }

    pub fn pause(&self) {
        self.play.pause();
    }

    pub fn sink(&self) -> &Element {
        &self.gtksink
    }

    pub fn toggle(&self, state: bool) {
        if state {
            eprintln!("Playing.");
            self.play();
        } else {
            eprintln!("Paused");
            self.pause();
        }
    }
}
