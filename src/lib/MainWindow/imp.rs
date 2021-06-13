use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct MainWindow;

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "VisiviMainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_title(Some("Visivi"))
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}