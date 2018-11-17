// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Misc(Object<ffi::AtkMisc, ffi::AtkMiscClass>);

    match fn {
        get_type => || ffi::atk_misc_get_type(),
    }
}

impl Misc {
    pub fn get_instance() -> Option<Misc> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::atk_misc_get_instance())
        }
    }
}

pub trait AtkMiscExt {
    fn threads_enter(&self);

    fn threads_leave(&self);
}

impl<O: IsA<Misc>> AtkMiscExt for O {
    fn threads_enter(&self) {
        unsafe {
            ffi::atk_misc_threads_enter(self.to_glib_none().0);
        }
    }

    fn threads_leave(&self) {
        unsafe {
            ffi::atk_misc_threads_leave(self.to_glib_none().0);
        }
    }
}

impl fmt::Display for Misc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Misc")
    }
}
