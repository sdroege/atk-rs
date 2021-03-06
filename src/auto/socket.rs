// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Component;
use Object;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Socket(Object<ffi::AtkSocket, ffi::AtkSocketClass>): Object, Component;

    match fn {
        get_type => || ffi::atk_socket_get_type(),
    }
}

impl Socket {
    pub fn new() -> Socket {
        assert_initialized_main_thread!();
        unsafe {
            Object::from_glib_full(ffi::atk_socket_new()).downcast_unchecked()
        }
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AtkSocketExt: 'static {
    fn embed(&self, plug_id: &str);

    fn is_occupied(&self) -> bool;
}

impl<O: IsA<Socket>> AtkSocketExt for O {
    fn embed(&self, plug_id: &str) {
        unsafe {
            ffi::atk_socket_embed(self.to_glib_none().0, plug_id.to_glib_none().0);
        }
    }

    fn is_occupied(&self) -> bool {
        unsafe {
            from_glib(ffi::atk_socket_is_occupied(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket")
    }
}
