#![allow(dead_code)]

extern crate gtk;
extern crate flac;

pub mod file;
pub mod gui;

use gui::Application;

fn main() {

    let app = Application::new();
    app.run();

}
