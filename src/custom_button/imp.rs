use gtk::glib;
use gtk::subclass::prelude::*;

// object to hold state
#[derive(Default)]
pub struct CustomButton;

// central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGTKappCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// trait shared by all GObjects
impl ObjectImpl for CustomButton {}

//trait shared by all widgets
impl WidgetImpl for CustomButton {}

// trait shared by all buttons
impl ButtonImpl for CustomButton {}
