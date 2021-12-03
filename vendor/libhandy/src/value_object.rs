use glib;
use glib::translate::*;
use ValueObject;

impl ValueObject {
    #[cfg(any(feature = "v0_0_8", feature = "dox"))]
    pub fn new_string(string: &str) -> ValueObject {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(handy_sys::hdy_value_object_new_take_string(
                string.to_glib_full(),
            ))
        }
    }
}

pub trait ValueObjectExtManual {
    #[cfg(any(feature = "v0_0_8", feature = "dox"))]
    fn copy_value(&self) -> glib::Value;
}

// impl<O: IsA<ValueObject> + IsA<glib::object::Object>> ValueObjectExtManual for O {
//     #[cfg(any(feature = "v0_0_8", feature = "dox"))]
//     fn copy_value(&self) -> glib::Value {
//         unsafe {
//             let mut dest = glib::Value::uninitialized();
//             ffi::hdy_value_object_copy_value(self.as_ref().to_glib_none().0, dest.to_glib_none_mut().0);
//             dest
//         }
//     }
// }
