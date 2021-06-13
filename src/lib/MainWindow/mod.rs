mod imp;

use glib::Object;
use gtk::glib;
use gtk::Application;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget;
}

impl MainWindow {
    pub fn new(application: &Application) -> Self {
        Object::new(&[("application", &application)]).expect("Failed to create MainWindow")
    }
}
