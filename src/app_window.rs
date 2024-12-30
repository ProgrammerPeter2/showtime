use gst::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;

use crate::gst_backend::GstBackend;
use adw::Application;
use glib::clone;
use glib::Object;
use gtk::prelude::*;
use gtk::{gio, Button, ProgressBar};
use std::cell::Cell;
use std::env;

mod imp {
    use super::*;

    use gstgtk4::RenderWidget;
    use gtk::subclass::prelude::*;
    use gtk::{glib, Box, CompositeTemplate};

    #[derive(CompositeTemplate)]
    #[template(resource = "/hu/peterhorvath/showtime/app_window.ui")]
    pub struct ShowtimeAppWindow {
        #[template_child]
        button: TemplateChild<Button>,
        #[template_child]
        progress_bar: TemplateChild<ProgressBar>,
        #[template_child]
        video_box: TemplateChild<Box>,
        video_widget: RenderWidget,
        state: Cell<bool>,
        receiver: async_channel::Receiver<u64>,
        player: GstBackend,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShowtimeAppWindow {
        const NAME: &'static str = "ShowtimeAppWindow";
        type Type = super::ShowtimeAppWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            let (sender, receiver) = async_channel::bounded::<u64>(1);
            let player = GstBackend::new(sender);
            Self {
                button: TemplateChild::default(),
                progress_bar: TemplateChild::default(),
                video_box: TemplateChild::default(),
                video_widget: RenderWidget::new(&player.sink()),
                state: Cell::new(false),
                receiver,
                player,
            }
        }
    }

    impl ObjectImpl for ShowtimeAppWindow {
        fn constructed(&self) {
            self.parent_constructed();

            match env::current_dir() {
                Ok(path) => {
                    let path: String = format!(
                        "file://{}",
                        path.with_file_name("video.mp4")
                            .to_str()
                            .expect("Failed to parse path!")
                    );
                    self.player.set_song_uri(Some(path.as_str()));
                }
                Err(err) => eprintln!("{:?}", err),
            }

            glib::spawn_future_local(clone!(
                #[weak(rename_to = progress_bar)]
                self.progress_bar,
                #[strong(rename_to = receiver)]
                self.receiver,
                async move {
                    while let Ok(position) = receiver.recv().await {
                        progress_bar.set_fraction(position as f64 / 157.0);
                    }
                }
            ));

            self.button.connect_clicked(clone!(
                #[weak(rename_to = this)]
                self,
                move |btn| {
                    this.button_clicked(btn);
                }
            ));

            self.video_widget.set_size_request(640, 480);
            self.video_box.append(&self.video_widget);
        }
    }

    impl ShowtimeAppWindow {
        fn button_clicked(&self, button: &Button) {
            self.state.set(!self.state.get());
            if self.state.get() {
                button.set_label("Pause");
            } else {
                button.set_label("Play");
            }
            self.player.toggle(self.state.get());
        }

        pub fn sink(&self) -> &gst::Element {
            self.player.sink()
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for ShowtimeAppWindow {}

    // Trait shared by all windows
    impl WindowImpl for ShowtimeAppWindow {}

    // Trait shared by all application windows
    impl ApplicationWindowImpl for ShowtimeAppWindow {}
}

glib::wrapper! {
    pub struct ShowtimeAppWindow(ObjectSubclass<imp::ShowtimeAppWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ShowtimeAppWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    pub fn sink(&self) -> &gst::Element {
        self.imp().sink()
    }
}
