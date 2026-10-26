use std::cell::OnceCell;
use adw::prelude::*;
use gettextrs::gettext;
use gtk::{
    gio,
    glib::{self, clone},
    subclass::prelude::*,
    prelude::*,
};

use crate::{
    application::Application,
};
mod imp {
    use adw::subclass::prelude::*;
    use glib::subclass;
    
    use super::*;

    #[derive(Default,gtk::CompositeTemplate, glib::Properties)]
    //org/example/myapp
    #[template(resource = "/org/example/myapp/window.ui")]
  
    #[properties(wrapper_type = super::Window)]
    pub struct Window {
       // #[property(get, set, construct_only)]
       //pub model: OnceCell<ProvidersModel>,
        //#[template_child]
       // pub main_stack: TemplateChild<gtk::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;
        type Interfaces = (gio::Initable,);

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            //klass.bind_template_instance_callbacks();

            
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            let win = self.obj();
        }
    }
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {
        fn enable_debugging(&self, toggle: bool) -> bool {
            false
        }

    
    }

    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
    impl InitableImpl for Window {
        fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
            let win = self.obj();
            
          
            Ok(())
        }
    }
}
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::Initable, gio::ActionMap, gio::ActionGroup, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::ShortcutManager,
                    gtk::Native, gtk::Root;
}
impl Window {
    pub fn new(app: &Application) -> Self {
        gio::Initable::builder()
            .property("application", app)
            .build(gio::Cancellable::NONE)
            .unwrap()
    }
}
