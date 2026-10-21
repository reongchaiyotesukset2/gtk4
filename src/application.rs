use gtk::{gio,
    glib::{self, clone},
    subclass::prelude::*,

};
use adw::prelude::*;
mod imp {

use adw::subclass::prelude::*;
    use super::*;

    #[derive(Default, glib::Properties)]    #[propert>
    pub struct Application {}


        #[glib::object_subclass]
        impl ObjectSubclass for Application {
            const NAME: &'static str = "Application";
            type ParentType = adw::Application;
            type Type = super::Application;

        }
        impl ObjectImpl for Application {}
        impl ApplicationImpl for Application{}
        impl AdwApplicationImpl for Application{}
        impl GtkApplicationImpl for Application{}
    impl Application{

    }
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Applic>

impl Application{
pub fn run(){
    println!("new");
}
}
