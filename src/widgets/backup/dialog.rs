use adw::prelude::*;
use anyhow::Result;
use gettextrs::gettext;
use gtk::{
    gio,
    glib::{self, clone},
    subclass::prelude::*,
};
mod imp {
    use std::{
        cell::{OnceCell, RefCell},
        collections::HashMap,
        sync::LazyLock,
    };

    use adw::subclass::prelude::*;
    use glib::subclass::Signal;

    use super::*;

    #[derive(gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::BackupDialog)]
    #[template(resource = "/org/example/myapp/backup_dialog.ui")]
    pub struct BackupDialog {
        
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BackupDialog {
        const NAME: &'static str = "BackupDialog";
        type Type = super::BackupDialog;
        type ParentType = adw::PreferencesDialog;

        fn new() -> Self {
            

            Self {
              
        
  
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BackupDialog {
        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> =
                LazyLock::new(|| vec![Signal::builder("restore-completed").action().build()]);
            SIGNALS.as_ref()
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

          
        }
    }
    impl WidgetImpl for BackupDialog {}
    impl AdwDialogImpl for BackupDialog {}
    impl PreferencesDialogImpl for BackupDialog {}
}

glib::wrapper! {
    pub struct BackupDialog(ObjectSubclass<imp::BackupDialog>)
        @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl BackupDialog {
  /*
    pub fn new(model: &ProvidersModel) -> Self {
        glib::Object::builder().property("model", model).build()
    }
*/
}