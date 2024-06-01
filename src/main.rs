use crate::glib::clone;
use gtk::{prelude::*, Orientation};
use gtk::{glib, Application, ApplicationWindow, Button};
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
    let number = Rc::new(Cell::new(0));
    // Create a button with label and margins
    let button_increase = Button::builder()
        .label("increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
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

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present window
    window.present();
}
