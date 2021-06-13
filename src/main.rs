use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{Application, CssProvider, StyleContext};
use visivi::MainWindow::MainWindow;

fn main() {
    let app = Application::new(Some("me.sergeykuroedov.visivi"), Default::default());
    app.connect_activate(|app| {
        setup_css();
        build_ui(app);
    });
    app.run();
}

fn build_ui(application: &Application) {
    let window = MainWindow::new(application)
        .present();
}

fn setup_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("res/style.css"));
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}