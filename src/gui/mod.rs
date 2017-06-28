/// This module contain all the gui code.
use gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType, ListStore};

/// The application.
pub struct Application {
    /// The top level GTK window.
    root_window: Window,

    file_list: ListStore,
}

impl Application {
    /// Initialize the Application.
    pub fn new() -> Application {

        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        } else {
            // Top level window
            let window = Window::new(WindowType::Toplevel);
            window.set_title("First GTK+ Program");
            window.set_default_size(350, 70);
            window.show_all();
            window.connect_delete_event(|_, _| {
                gtk::main_quit();
                Inhibit(false)
            });
            Application {
                root_window: window,
                file_list: ListStore {
                    0: "rien",
                    1: "ok",
                },
            }
        }
    }

    /// Launch GTK event loop.
    pub fn run(&self) {
        gtk::main();
    }
}
