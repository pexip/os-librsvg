// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use SocketConnection;
use SocketListener;
use ffi;
use glib;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SocketService(Object<ffi::GSocketService, ffi::GSocketServiceClass>): SocketListener;

    match fn {
        get_type => || ffi::g_socket_service_get_type(),
    }
}

impl SocketService {
    pub fn new() -> SocketService {
        unsafe {
            from_glib_full(ffi::g_socket_service_new())
        }
    }
}

impl Default for SocketService {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SocketServiceExt {
    fn is_active(&self) -> bool;

    fn start(&self);

    fn stop(&self);

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn get_property_active(&self) -> bool;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn set_property_active(&self, active: bool);

    fn connect_incoming<F: Fn(&Self, &SocketConnection, &Option<glib::Object>) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketService> + IsA<glib::object::Object>> SocketServiceExt for O {
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_service_is_active(self.to_glib_none().0))
        }
    }

    fn start(&self) {
        unsafe {
            ffi::g_socket_service_start(self.to_glib_none().0);
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::g_socket_service_stop(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "active".to_glib_none().0, Value::from(&active).to_glib_none().0);
        }
    }

    fn connect_incoming<F: Fn(&Self, &SocketConnection, &Option<glib::Object>) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &SocketConnection, &Option<glib::Object>) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "incoming",
                transmute(incoming_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn incoming_trampoline<P>(this: *mut ffi::GSocketService, connection: *mut ffi::GSocketConnection, source_object: *mut gobject_ffi::GObject, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<SocketService> {
    callback_guard!();
    let f: &&(Fn(&P, &SocketConnection, &Option<glib::Object>) -> bool + 'static) = transmute(f);
    f(&SocketService::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(connection), &from_glib_borrow(source_object)).to_glib()
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GSocketService, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketService> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketService::from_glib_borrow(this).downcast_unchecked())
}
