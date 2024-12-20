use async_channel::Sender;
use glib::clone;
use gst::prelude::*;
use gtk4::glib;

#[derive(Debug, Clone)]
pub struct GstBackend {
    sender: Sender<u64>,
    gst_player: gst_play::Play,
    gst_signals: gst_play::PlaySignalAdapter,
}

impl GstBackend {
    pub fn new(sender: Sender<u64>) -> Self {
        let gst_player = gst_play::Play::default();
        gst_player.set_video_track_enabled(false);

        let signals = gst_play::PlaySignalAdapter::new(&gst_player);
        let res = Self {
            sender,
            gst_player,
            gst_signals: signals,
        };
        res.setup_signals();

        res
    }

    fn setup_signals(&self) {
        self.gst_signals.connect_position_updated(clone!(
            #[strong(rename_to = sender)]
            self.sender,
            move |_, clock| {
                if let Some(clock) = clock {
                    let position = clock.seconds() as u64;
                    if let Err(e) = sender.send_blocking(position) {
                        eprintln!("Failed to send position: {}", e);
                    }
                }
            }
        ));
    }

    pub fn set_song_uri(&self, uri: Option<&str>) {
        // FIXME: https://gitlab.freedesktop.org/gstreamer/gstreamer/-/issues/1124
        if uri.is_some() {
            self.gst_player.set_uri(uri);
        }
    }

    pub fn play(&self) {
        self.gst_player.play();
    }

    pub fn pause(&self) {
        self.gst_player.pause();
    }

    pub fn stop(&self) {
        self.gst_player.stop();
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
