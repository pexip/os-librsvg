// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TabAlign;
use glib::translate::*;
#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
use std::fmt;
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TabArray(Boxed<ffi::PangoTabArray>);

    match fn {
        copy => |ptr| ffi::pango_tab_array_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_tab_array_free(ptr),
        type_ => || ffi::pango_tab_array_get_type(),
    }
}

impl TabArray {
    #[doc(alias = "pango_tab_array_new")]
    pub fn new(initial_size: i32, positions_in_pixels: bool) -> TabArray {
        unsafe {
            from_glib_full(ffi::pango_tab_array_new(
                initial_size,
                positions_in_pixels.into_glib(),
            ))
        }
    }

    //#[doc(alias = "pango_tab_array_new_with_positions")]
    //#[doc(alias = "new_with_positions")]
    //pub fn with_positions(size: i32, positions_in_pixels: bool, first_alignment: TabAlign, first_position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TabArray {
    //    unsafe { TODO: call ffi:pango_tab_array_new_with_positions() }
    //}

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_tab_array_get_decimal_point")]
    #[doc(alias = "get_decimal_point")]
    pub fn decimal_point(&self, tab_index: i32) -> char {
        unsafe {
            std::convert::TryFrom::try_from(ffi::pango_tab_array_get_decimal_point(
                mut_override(self.to_glib_none().0),
                tab_index,
            ))
            .expect("conversion from an invalid Unicode value attempted")
        }
    }

    #[doc(alias = "pango_tab_array_get_positions_in_pixels")]
    #[doc(alias = "get_positions_in_pixels")]
    pub fn is_positions_in_pixels(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_tab_array_get_positions_in_pixels(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "pango_tab_array_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&mut self) -> i32 {
        unsafe { ffi::pango_tab_array_get_size(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "pango_tab_array_get_tab")]
    #[doc(alias = "get_tab")]
    pub fn tab(&self, tab_index: i32) -> (TabAlign, i32) {
        unsafe {
            let mut alignment = mem::MaybeUninit::uninit();
            let mut location = mem::MaybeUninit::uninit();
            ffi::pango_tab_array_get_tab(
                mut_override(self.to_glib_none().0),
                tab_index,
                alignment.as_mut_ptr(),
                location.as_mut_ptr(),
            );
            let alignment = alignment.assume_init();
            let location = location.assume_init();
            (from_glib(alignment), location)
        }
    }

    //#[doc(alias = "pango_tab_array_get_tabs")]
    //#[doc(alias = "get_tabs")]
    //pub fn tabs(&mut self, locations: Vec<i32>) -> TabAlign {
    //    unsafe { TODO: call ffi:pango_tab_array_get_tabs() }
    //}

    #[doc(alias = "pango_tab_array_resize")]
    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            ffi::pango_tab_array_resize(self.to_glib_none_mut().0, new_size);
        }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_tab_array_set_decimal_point")]
    pub fn set_decimal_point(&mut self, tab_index: i32, decimal_point: char) {
        unsafe {
            ffi::pango_tab_array_set_decimal_point(
                self.to_glib_none_mut().0,
                tab_index,
                decimal_point.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_tab_array_set_positions_in_pixels")]
    pub fn set_positions_in_pixels(&mut self, positions_in_pixels: bool) {
        unsafe {
            ffi::pango_tab_array_set_positions_in_pixels(
                self.to_glib_none_mut().0,
                positions_in_pixels.into_glib(),
            );
        }
    }

    #[doc(alias = "pango_tab_array_set_tab")]
    pub fn set_tab(&mut self, tab_index: i32, alignment: TabAlign, location: i32) {
        unsafe {
            ffi::pango_tab_array_set_tab(
                self.to_glib_none_mut().0,
                tab_index,
                alignment.into_glib(),
                location,
            );
        }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_tab_array_sort")]
    pub fn sort(&mut self) {
        unsafe {
            ffi::pango_tab_array_sort(self.to_glib_none_mut().0);
        }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_tab_array_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::pango_tab_array_to_string(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_tab_array_from_string")]
    pub fn from_string(text: &str) -> Result<TabArray, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::pango_tab_array_from_string(text.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Can't parse a TabArray"))
        }
    }
}

#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
impl fmt::Display for TabArray {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

unsafe impl Send for TabArray {}
unsafe impl Sync for TabArray {}
