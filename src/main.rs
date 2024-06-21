mod custom_button;

use custom_button::CustomButton;

use crate::glib::clone;
use gtk::{
    glib, Application, ApplicationWindow, Button, Label, ListBox, PolicyType, ScrolledWindow,
    Switch,
};
use gtk::{prelude::*, Orientation};
use soloud::*;
use std::cell::Cell;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let switch = Switch::new();

    // Set and then immediately obtain active property

    let number = Rc::new(Cell::new(0));
    // Create a button with label and margins
    let button_increase = CustomButton::with_label("increase");
    button_increase.set_margin_top(12);
    button_increase.set_margin_bottom(12);
    button_increase.set_margin_start(12);
    button_increase.set_margin_end(12);

    let button_decrease = Button::builder()
        .label("decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);
    // Connect callbacks
    // When a button is clicked, `number` should be changed
    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed

    // Set and then immediately obtain active property
    switch.set_active(true);
    let switch_active = switch.is_active();

    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>

        move |_| {
            button_decrease.set_label(&number.get().to_string());
            number.set(number.get() + 1);

            switch.set_active(true);
            let switch_active = switch.is_active();

            // This prints: "The active property of switch is true"
            println!("The active property of switch is {}", switch_active);
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));
    // Connect to "clicked" signal of `button`
    // button.connect_clicked(|button| {
    //     // Set the label to "Hello World!" after the button has been clicked on
    //     button.set_label("Hello World!");

    //     let sl = Soloud::default().unwrap();
    //     let mut wav = audio::Wav::default();
    //     wav.load_mem(include_bytes!("../assets/Laid-Off.mp3"))
    //         .unwrap();
    //     sl.play(&wav);
    //     while sl.voice_count() > 0 {
    //         std::thread::sleep(std::time::Duration::from_millis(100));
    //     }
    // });

    let switch = Switch::new();
    switch.set_margin_top(42);
    switch.set_margin_bottom(12);
    switch.set_margin_start(12);
    switch.set_margin_end(12);

    let list_box = ListBox::new();
    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }

    // list_box is long and needs a scrolled window
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_box)
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&switch);

    // This prints: "The active property of switch is true"
    // println!("The active property of switch is {}", switch_active);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&scrolled_window)
        .child(&gtk_box)
        .build();

    // Present window
    window.present();
}
