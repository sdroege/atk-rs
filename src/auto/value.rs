// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_12", feature = "dox"))]
use Range;
use ffi;
use glib;
#[cfg(any(feature = "v2_12", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_12", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_12", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_12", feature = "dox"))]
use libc;
#[cfg(any(feature = "v2_12", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
#[cfg(any(feature = "v2_12", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Value(Object<ffi::AtkValue, ffi::AtkValueIface>);

    match fn {
        get_type => || ffi::atk_value_get_type(),
    }
}

pub trait ValueExt {
    fn get_current_value(&self) -> glib::Value;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_increment(&self) -> f64;

    fn get_maximum_value(&self) -> glib::Value;

    fn get_minimum_increment(&self) -> glib::Value;

    fn get_minimum_value(&self) -> glib::Value;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_range(&self) -> Option<Range>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_sub_ranges(&self) -> Vec<Range>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_value_and_text(&self) -> (f64, String);

    fn set_current_value(&self, value: &glib::Value) -> bool;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn set_value(&self, new_value: f64);

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn connect_value_changed<F: Fn(&Self, f64, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Value> + IsA<glib::object::Object>> ValueExt for O {
    fn get_current_value(&self) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::atk_value_get_current_value(self.to_glib_none().0, value.to_glib_none_mut().0);
            value
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_increment(&self) -> f64 {
        unsafe {
            ffi::atk_value_get_increment(self.to_glib_none().0)
        }
    }

    fn get_maximum_value(&self) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::atk_value_get_maximum_value(self.to_glib_none().0, value.to_glib_none_mut().0);
            value
        }
    }

    fn get_minimum_increment(&self) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::atk_value_get_minimum_increment(self.to_glib_none().0, value.to_glib_none_mut().0);
            value
        }
    }

    fn get_minimum_value(&self) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::atk_value_get_minimum_value(self.to_glib_none().0, value.to_glib_none_mut().0);
            value
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_range(&self) -> Option<Range> {
        unsafe {
            from_glib_full(ffi::atk_value_get_range(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_sub_ranges(&self) -> Vec<Range> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::atk_value_get_sub_ranges(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_value_and_text(&self) -> (f64, String) {
        unsafe {
            let mut value = mem::uninitialized();
            let mut text = ptr::null_mut();
            ffi::atk_value_get_value_and_text(self.to_glib_none().0, &mut value, &mut text);
            (value, from_glib_full(text))
        }
    }

    fn set_current_value(&self, value: &glib::Value) -> bool {
        unsafe {
            from_glib(ffi::atk_value_set_current_value(self.to_glib_none().0, value.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn set_value(&self, new_value: f64) {
        unsafe {
            ffi::atk_value_set_value(self.to_glib_none().0, new_value);
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn connect_value_changed<F: Fn(&Self, f64, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "value-changed",
                transmute(value_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
unsafe extern "C" fn value_changed_trampoline<P>(this: *mut ffi::AtkValue, value: libc::c_double, text: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Value> {
    let f: &&(Fn(&P, f64, &str) + 'static) = transmute(f);
    f(&Value::from_glib_borrow(this).downcast_unchecked(), value, &String::from_glib_none(text))
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value")
    }
}
