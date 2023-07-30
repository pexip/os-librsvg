// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::translate::*;
use crate::BindingFlags;
use std::fmt;

crate::wrapper! {
    #[doc(alias = "GBinding")]
    pub struct Binding(Object<gobject_ffi::GBinding>);

    match fn {
        type_ => || gobject_ffi::g_binding_get_type(),
    }
}

impl Binding {
    #[doc(alias = "g_binding_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> BindingFlags {
        unsafe { from_glib(gobject_ffi::g_binding_get_flags(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_binding_get_source_property")]
    #[doc(alias = "get_source_property")]
    pub fn source_property(&self) -> crate::GString {
        unsafe {
            from_glib_none(gobject_ffi::g_binding_get_source_property(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_binding_get_target_property")]
    #[doc(alias = "get_target_property")]
    pub fn target_property(&self) -> crate::GString {
        unsafe {
            from_glib_none(gobject_ffi::g_binding_get_target_property(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_binding_unbind")]
    pub fn unbind(&self) {
        unsafe {
            gobject_ffi::g_binding_unbind(self.to_glib_none().0);
        }
    }
}

unsafe impl Send for Binding {}
unsafe impl Sync for Binding {}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Binding")
    }
}
