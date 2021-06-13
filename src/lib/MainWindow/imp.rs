use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct MainWindowImpl;

#[glib::object_subclass]
impl ObjectSubclass for MainWindowImpl {
    const NAME: &'static str = "VisiviMainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for MainWindowImpl {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_title(Some("Visivi"))
    }
}

impl WidgetImpl for MainWindowImpl {}

impl WindowImpl for MainWindowImpl {}

impl ApplicationWindowImpl for MainWindowImpl {}