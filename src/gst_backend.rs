use glib::clone;
use gst::prelude::*;
use gtk4::glib;

#[derive(Debug, Clone)]
pub struct GstBackend {
    gst_player: gst_play::Play,
}

impl GstBackend {
    pub fn new() -> Self {
        let gst_player = gst_play::Play::default();
        gst_player.set_video_track_enabled(false);
        let res = Self { gst_player };
        res
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
