// Take a look at the license at the top of the repository in the LICENSE file.

//! Module for registering boxed types for Rust types.

use crate::translate::*;
use crate::StaticType;

/// Trait for defining boxed types.
///
/// Links together the type name with the type itself.
///
/// See [`register_boxed_type`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_boxed_type`]: fn.register_boxed_type.html
pub trait BoxedType: StaticType + Clone + Sized + 'static {
    /// Boxed type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;
}

/// Register a boxed `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// See [`GBoxed!`] for defining a function that ensures that
/// this is only called once and returns the type id.
///
/// [`GBoxed!`]: ../../derive.GBoxed.html
pub fn register_boxed_type<T: BoxedType>() -> crate::Type {
    unsafe extern "C" fn boxed_copy<T: BoxedType>(v: ffi::gpointer) -> ffi::gpointer {
        let v = &*(v as *mut T);
        let copy = Box::new(v.clone());

        Box::into_raw(copy) as ffi::gpointer
    }
    unsafe extern "C" fn boxed_free<T: BoxedType>(v: ffi::gpointer) {
        let v = v as *mut T;
        let _ = Box::from_raw(v);
    }
    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();
        assert_eq!(
            gobject_ffi::g_type_from_name(type_name.as_ptr()),
            gobject_ffi::G_TYPE_INVALID,
            "Type {} has already been registered",
            type_name.to_str().unwrap()
        );

        from_glib(gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(boxed_copy::<T>),
            Some(boxed_free::<T>),
        ))
    }
}

#[cfg(test)]
mod test {
    // We rename the current crate as glib, since the macros in glib-macros
    // generate the glib namespace through the crate_ident_new utility,
    // and that returns `glib` (and not `crate`) when called inside the glib crate
    use crate as glib;
    use crate::value::ToValue;
    use crate::StaticType;

    #[derive(Clone, Debug, PartialEq, Eq, glib::GBoxed)]
    #[gboxed(type_name = "MyBoxed")]
    struct MyBoxed(String);

    #[test]
    fn test_register() {
        assert!(MyBoxed::static_type().is_valid());
    }

    #[test]
    fn test_value() {
        assert!(MyBoxed::static_type().is_valid());

        let b = MyBoxed(String::from("abc"));
        let v = b.to_value();
        let b2 = v.get::<&MyBoxed>().unwrap();
        assert_eq!(&b, b2);
    }
}
