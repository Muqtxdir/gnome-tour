use glib::subclass::prelude::*;

use ApplicationWindowClass;

pub trait ApplicationWindowImpl:
    gtk::subclass::application_window::ApplicationWindowImpl + 'static
{
}

unsafe impl<T: ObjectSubclass + ApplicationWindowImpl> IsSubclassable<T>
    for ApplicationWindowClass
{
    fn override_vfuncs(&mut self) {
        <gtk::ApplicationWindowClass as IsSubclassable<T>>::override_vfuncs(self);
    }
}
