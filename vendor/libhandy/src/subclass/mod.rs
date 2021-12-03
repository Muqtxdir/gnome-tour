pub mod application_window;

pub mod prelude {
    pub use super::application_window::ApplicationWindowImpl;
    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
}
