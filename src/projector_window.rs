use glib::subclass::types::ObjectSubclassIsExt;
use glib::Object;
use gtk::{gio, glib};

mod imp {
    use adw::subclass::window::AdwWindowImpl;
    use gst::subclass::prelude::ObjectImplExt;
    use gstgtk4::RenderWidget;
    use gtk::prelude::BoxExt;
    use gtk::subclass::prelude::*;
    use gtk::{glib, Box, CompositeTemplate, Orientation, TemplateChild};
    use std::cell::RefCell;

    #[derive(CompositeTemplate)]
    #[template(resource = "/hu/peterhorvath/showtime/projector_window.ui")]
    pub struct ShowtimeProjectorWindow {
        #[template_child]
        video_holder: TemplateChild<Box>,
        video_widget: RefCell<Option<RenderWidget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShowtimeProjectorWindow {
        const NAME: &'static str = "ShowtimeProjectorWindow";
        type Type = super::ShowtimeProjectorWindow;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                video_holder: TemplateChild::default(),
                video_widget: RefCell::new(None),
            }
        }
    }

    impl ObjectImpl for ShowtimeProjectorWindow {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl ShowtimeProjectorWindow {
        pub fn setup_player(&self, sink: &gst::Element) {
            use gtk::prelude::WidgetExt;

            let video_widget = gstgtk4::RenderWidget::new(sink);
            video_widget.set_size_request(1250, 720);
            self.video_holder.append(&video_widget);
            self.video_widget.replace(Some(video_widget));
        }
    }

    impl WidgetImpl for ShowtimeProjectorWindow {}

    impl WindowImpl for ShowtimeProjectorWindow {}

    impl AdwWindowImpl for ShowtimeProjectorWindow {}
}

glib::wrapper! {
    pub struct ShowtimeProjectorWindow(ObjectSubclass<imp::ShowtimeProjectorWindow>)
        @extends adw::Window, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                                gtk::ConstraintTarget, gtk::Native, gtk::Root;
}

impl ShowtimeProjectorWindow {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn setup_player(&self, sink: &gst::Element) {
        self.imp().setup_player(sink);
    }
}
