extern crate gtk;
extern crate libhandy;
use libhandy::ClampExt;

#[test]
fn check_build() {
    assert!(gtk::init().is_ok());
    let column = libhandy::Clamp::new();
    column.set_maximum_size(800);
    column.set_tightening_threshold(600);
}
