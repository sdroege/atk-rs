// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ObjectFactory;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct NoOpObjectFactory(Object<ffi::AtkNoOpObjectFactory, ffi::AtkNoOpObjectFactoryClass>): ObjectFactory;

    match fn {
        get_type => || ffi::atk_no_op_object_factory_get_type(),
    }
}

impl NoOpObjectFactory {
    pub fn new() -> NoOpObjectFactory {
        assert_initialized_main_thread!();
        unsafe {
            ObjectFactory::from_glib_full(ffi::atk_no_op_object_factory_new()).downcast_unchecked()
        }
    }
}

impl Default for NoOpObjectFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NoOpObjectFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NoOpObjectFactory")
    }
}
