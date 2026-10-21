use gtk::{gio, glib};

mod application;

use application::Application;

fn main(){
    gtk::init().expect("failed to init gtk");

  //  let res = gio::Resource::load(config::PKGDATADI>
        //.expect("Could not load resources");
    //gio::resources_register(&res);

   // glib::set_application_name(&gettext("Authentica>

    Application::run()
}
