fn main() {
    glib_build_tools::compile_resources(
        &["res/ui", "res/icons", "res/styles", "res"],
        "res/resources.gresource.xml",
        "showtime.gresource",
    );
}
