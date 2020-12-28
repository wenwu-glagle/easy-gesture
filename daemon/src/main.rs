extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::Application;
use x11::xlib::{XOpenDisplay, XDefaultRootWindow};

mod grabber;
use grabber::Grabber;

fn main() {
    let application = Application::new(
        Some("easygesture.daemon"),
        Default::default(),
    )
    .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        unsafe {
            let display = XOpenDisplay(std::ptr::null());
            let root = XDefaultRootWindow(display);

            let grabber = Grabber::new(display, root);
            grabber.force_generate_enter_events();
        }

        app.hold();
    });

    application.run(&[]);
}
