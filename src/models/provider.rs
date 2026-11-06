use std::{
    string::ToString,
    time::{SystemTime, UNIX_EPOCH},
};

//use anyhow::Result;
//use diesel::prelude::*;
use gtk::{
    gdk_pixbuf, gio,
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};

mod imp {
  use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::Provider)]
    pub struct Provider {
        #[property(get, set, construct_only)]
        pub id: Cell<u32>,
        #[property(get, set)]
        pub name: RefCell<String>,
    }
  #[glib::object_subclass]
    impl ObjectSubclass for Provider {
        const NAME: &'static str = "Provider";
        type Type = super::Provider;

        fn new() -> Self {
          Self{
            id: Cell::default(),
            name: RefCell::default(),
          }
        }
}
  #[glib::derived_properties]
    impl ObjectImpl for Provider {
        fn dispose(&self) {
            
            }
        }
      impl Provider {
      }
}
glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>);
}

impl Provider {
}