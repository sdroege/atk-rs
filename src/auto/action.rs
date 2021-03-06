// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Action(Object<ffi::AtkAction, ffi::AtkActionIface>);

    match fn {
        get_type => || ffi::atk_action_get_type(),
    }
}

pub trait AtkActionExt: 'static {
    fn do_action(&self, i: i32) -> bool;

    fn get_description(&self, i: i32) -> Option<GString>;

    fn get_keybinding(&self, i: i32) -> Option<GString>;

    fn get_localized_name(&self, i: i32) -> Option<GString>;

    fn get_n_actions(&self) -> i32;

    fn get_name(&self, i: i32) -> Option<GString>;

    fn set_description(&self, i: i32, desc: &str) -> bool;
}

impl<O: IsA<Action>> AtkActionExt for O {
    fn do_action(&self, i: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_action_do_action(self.to_glib_none().0, i))
        }
    }

    fn get_description(&self, i: i32) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::atk_action_get_description(self.to_glib_none().0, i))
        }
    }

    fn get_keybinding(&self, i: i32) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::atk_action_get_keybinding(self.to_glib_none().0, i))
        }
    }

    fn get_localized_name(&self, i: i32) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::atk_action_get_localized_name(self.to_glib_none().0, i))
        }
    }

    fn get_n_actions(&self) -> i32 {
        unsafe {
            ffi::atk_action_get_n_actions(self.to_glib_none().0)
        }
    }

    fn get_name(&self, i: i32) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::atk_action_get_name(self.to_glib_none().0, i))
        }
    }

    fn set_description(&self, i: i32, desc: &str) -> bool {
        unsafe {
            from_glib(ffi::atk_action_set_description(self.to_glib_none().0, i, desc.to_glib_none().0))
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}
