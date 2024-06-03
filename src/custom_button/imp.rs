use std::cell::Cell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// object to hold state
#[derive(Default)]
pub struct CustomButton {
    number: Cell<i32>,
}

// central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGTKappCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// trait shared by all GObjects
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_label(&self.number.get().to_string());
    }
}

//trait shared by all widgets
impl WidgetImpl for CustomButton {}

// trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        self.number.set(self.number.get() +1);
        self.obj().set_label(&self.number.get().to_string())
    }
}
