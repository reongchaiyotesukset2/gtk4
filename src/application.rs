use gtk::{
    gio,
    glib::{self},
    subclass::prelude::*,
    prelude::*,
};

use crate::{
   widgets::{Window},
   
};

//use adw::prelude::*;
mod imp {
    use std::cell::{Cell, RefCell};
    use adw::subclass::prelude::*;
    use super::*;

    #[derive(Default, glib::Properties)]   
    #[properties(wrapper_type = super::Application)]
    pub struct Application {
       pub window: RefCell<Option<glib::WeakRef<Window>>>,
    }


       #[glib::object_subclass]
       impl ObjectSubclass for Application {
            const NAME: &'static str = "Application";
            type ParentType = adw::Application;
            type Type = super::Application;

        }
        impl ObjectImpl for Application {}
        impl ApplicationImpl for Application{
              fn startup(&self) {
                 self.parent_startup();
                  let app = self.obj();
                  let quit_action = gio::ActionEntry::builder("quit")
                  .activate(|app : &Self::Type, _, _| app.quit())
                  .build();
              }
              fn activate(&self) {
               
                 let app = self.obj();
                 let window = Window::new(&app);
                  window.present();
                  //self.window.replace(Some(window.downgrade()));
          }
            
        }
        impl AdwApplicationImpl for Application{}
        impl GtkApplicationImpl for Application{}
        impl Application{}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}
impl Application{

 pub fn run() -> glib::ExitCode {
     let app = glib::Object::builder::<Application>()
            .property("application-id", "gtk4db")
            .property("flags", gio::ApplicationFlags::HANDLES_OPEN)
            .build();
     app.run()
 }

    pub fn active_window(&self) -> Window {
        self.imp()
            .window
            .borrow()
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
    }

}
