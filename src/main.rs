#![allow(linker_messages)]
use gtk::{gio, glib};

mod application;
mod widgets;

use gettextrs::*;
use crate::application::Application;


fn main()->glib::ExitCode{
    gtk::init().expect("failed to init gtk");

    let res = gio::Resource::load("data/resources.gresource")
        .expect("Could not load resources");
     gio::resources_register(&res);
     glib::set_application_name("gtk4db");
    Application::run()
}
