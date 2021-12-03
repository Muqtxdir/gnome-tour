// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk;
use handy_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Keypad(Object<handy_sys::HdyKeypad, handy_sys::HdyKeypadClass, KeypadClass>) @extends gtk::Bin, gtk::Container, gtk::Widget, @implements gtk::Buildable;

    match fn {
        get_type => || handy_sys::hdy_keypad_get_type(),
    }
}

impl Keypad {
    pub fn new(symbols_visible: bool, letters_visible: bool) -> Keypad {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(handy_sys::hdy_keypad_new(
                symbols_visible.to_glib(),
                letters_visible.to_glib(),
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct KeypadBuilder {
    column_spacing: Option<u32>,
    end_action: Option<gtk::Widget>,
    entry: Option<gtk::Entry>,
    letters_visible: Option<bool>,
    row_spacing: Option<u32>,
    start_action: Option<gtk::Widget>,
    symbols_visible: Option<bool>,
    border_width: Option<u32>,
    child: Option<gtk::Widget>,
    resize_mode: Option<gtk::ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<gtk::Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<gtk::Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    //style: /*Unknown type*/,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<gtk::Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl KeypadBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Keypad {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref column_spacing) = self.column_spacing {
            properties.push(("column-spacing", column_spacing));
        }
        if let Some(ref end_action) = self.end_action {
            properties.push(("end-action", end_action));
        }
        if let Some(ref entry) = self.entry {
            properties.push(("entry", entry));
        }
        if let Some(ref letters_visible) = self.letters_visible {
            properties.push(("letters-visible", letters_visible));
        }
        if let Some(ref row_spacing) = self.row_spacing {
            properties.push(("row-spacing", row_spacing));
        }
        if let Some(ref start_action) = self.start_action {
            properties.push(("start-action", start_action));
        }
        if let Some(ref symbols_visible) = self.symbols_visible {
            properties.push(("symbols-visible", symbols_visible));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        let ret = glib::Object::new(Keypad::static_type(), &properties)
            .expect("object new")
            .downcast::<Keypad>()
            .expect("downcast");
        ret
    }

    pub fn column_spacing(mut self, column_spacing: u32) -> Self {
        self.column_spacing = Some(column_spacing);
        self
    }

    pub fn end_action<P: IsA<gtk::Widget>>(mut self, end_action: &P) -> Self {
        self.end_action = Some(end_action.clone().upcast());
        self
    }

    pub fn entry<P: IsA<gtk::Entry>>(mut self, entry: &P) -> Self {
        self.entry = Some(entry.clone().upcast());
        self
    }

    pub fn letters_visible(mut self, letters_visible: bool) -> Self {
        self.letters_visible = Some(letters_visible);
        self
    }

    pub fn row_spacing(mut self, row_spacing: u32) -> Self {
        self.row_spacing = Some(row_spacing);
        self
    }

    pub fn start_action<P: IsA<gtk::Widget>>(mut self, start_action: &P) -> Self {
        self.start_action = Some(start_action.clone().upcast());
        self
    }

    pub fn symbols_visible(mut self, symbols_visible: bool) -> Self {
        self.symbols_visible = Some(symbols_visible);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<gtk::Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: gtk::ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: gtk::Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<gtk::Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: gtk::Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_KEYPAD: Option<&Keypad> = None;

pub trait KeypadExt: 'static {
    fn get_column_spacing(&self) -> u32;

    fn get_end_action(&self) -> Option<gtk::Widget>;

    fn get_entry(&self) -> Option<gtk::Entry>;

    fn get_letters_visible(&self) -> bool;

    fn get_row_spacing(&self) -> u32;

    fn get_start_action(&self) -> Option<gtk::Widget>;

    fn get_symbols_visible(&self) -> bool;

    fn set_column_spacing(&self, spacing: u32);

    fn set_end_action<P: IsA<gtk::Widget>>(&self, end_action: Option<&P>);

    fn set_entry<P: IsA<gtk::Entry>>(&self, entry: Option<&P>);

    fn set_letters_visible(&self, letters_visible: bool);

    fn set_row_spacing(&self, spacing: u32);

    fn set_start_action<P: IsA<gtk::Widget>>(&self, start_action: Option<&P>);

    fn set_symbols_visible(&self, symbols_visible: bool);

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_end_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_letters_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_action_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_symbols_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Keypad>> KeypadExt for O {
    fn get_column_spacing(&self) -> u32 {
        unsafe { handy_sys::hdy_keypad_get_column_spacing(self.as_ref().to_glib_none().0) }
    }

    fn get_end_action(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(handy_sys::hdy_keypad_get_end_action(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_entry(&self) -> Option<gtk::Entry> {
        unsafe {
            from_glib_none(handy_sys::hdy_keypad_get_entry(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_letters_visible(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_keypad_get_letters_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_row_spacing(&self) -> u32 {
        unsafe { handy_sys::hdy_keypad_get_row_spacing(self.as_ref().to_glib_none().0) }
    }

    fn get_start_action(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(handy_sys::hdy_keypad_get_start_action(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_symbols_visible(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_keypad_get_symbols_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            handy_sys::hdy_keypad_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn set_end_action<P: IsA<gtk::Widget>>(&self, end_action: Option<&P>) {
        unsafe {
            handy_sys::hdy_keypad_set_end_action(
                self.as_ref().to_glib_none().0,
                end_action.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_entry<P: IsA<gtk::Entry>>(&self, entry: Option<&P>) {
        unsafe {
            handy_sys::hdy_keypad_set_entry(
                self.as_ref().to_glib_none().0,
                entry.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_letters_visible(&self, letters_visible: bool) {
        unsafe {
            handy_sys::hdy_keypad_set_letters_visible(
                self.as_ref().to_glib_none().0,
                letters_visible.to_glib(),
            );
        }
    }

    fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            handy_sys::hdy_keypad_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn set_start_action<P: IsA<gtk::Widget>>(&self, start_action: Option<&P>) {
        unsafe {
            handy_sys::hdy_keypad_set_start_action(
                self.as_ref().to_glib_none().0,
                start_action.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_symbols_visible(&self, symbols_visible: bool) {
        unsafe {
            handy_sys::hdy_keypad_set_symbols_visible(
                self.as_ref().to_glib_none().0,
                symbols_visible.to_glib(),
            );
        }
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdyKeypad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Keypad>,
        {
            let f: &F = &*(f as *const F);
            f(&Keypad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column-spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_end_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_action_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdyKeypad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Keypad>,
        {
            let f: &F = &*(f as *const F);
            f(&Keypad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::end-action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_end_action_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_entry_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdyKeypad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Keypad>,
        {
            let f: &F = &*(f as *const F);
            f(&Keypad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::entry\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_entry_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_letters_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_letters_visible_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdyKeypad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Keypad>,
        {
            let f: &F = &*(f as *const F);
            f(&Keypad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::letters-visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_letters_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdyKeypad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Keypad>,
        {
            let f: &F = &*(f as *const F);
            f(&Keypad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row-spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_start_action_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_action_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdyKeypad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Keypad>,
        {
            let f: &F = &*(f as *const F);
            f(&Keypad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start-action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_action_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_symbols_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_symbols_visible_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdyKeypad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Keypad>,
        {
            let f: &F = &*(f as *const F);
            f(&Keypad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::symbols-visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_symbols_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Keypad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Keypad")
    }
}