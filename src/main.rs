use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use visivi::MainWindow::MainWindow;

fn main() {
    let app = Application::new(Some("me.sergeykuroedov.visivi"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(application: &Application) {
    let window = MainWindow::new(application)
        .present();
}