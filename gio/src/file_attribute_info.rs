// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GFileAttributeInfo")]
    pub struct FileAttributeInfo(BoxedInline<ffi::GFileAttributeInfo>);

    match fn {
        copy => |ptr| {
            let copy = glib::ffi::g_malloc0(mem::size_of::<ffi::GFileAttributeInfo>()) as *mut ffi::GFileAttributeInfo;
            if !(*ptr).name.is_null() {
                (*copy).name = glib::ffi::g_strdup((*ptr).name);
            }
            copy
        },
        free => |ptr| {
            if !(*ptr).name.is_null() {
                glib::ffi::g_free((*ptr).name as *mut _);
            }
            glib::ffi::g_free(ptr as *mut _);
        },
        init => |ptr| {
            *ptr = mem::zeroed();
        },
        copy_into => |dest, src| {
            ptr::copy_nonoverlapping(src, dest, 1);
            if !(*dest).name.is_null() {
                (*dest).name = glib::ffi::g_strdup((*dest).name);
            }
        },
        clear => |ptr| {
            if !(*ptr).name.is_null() {
                glib::ffi::g_free((*ptr).name as *mut _);
            }
        },
    }
}

impl FileAttributeInfo {
    pub fn name(&self) -> &str {
        unsafe {
            use std::ffi::CStr;

            CStr::from_ptr(self.0.name)
                .to_str()
                .expect("non-UTF-8 string")
        }
    }

    pub fn type_(&self) -> crate::FileAttributeType {
        unsafe { from_glib(self.0.type_) }
    }

    pub fn flags(&self) -> crate::FileAttributeInfoFlags {
        unsafe { from_glib(self.0.flags) }
    }
}

impl fmt::Debug for FileAttributeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileAttributeInfo")
            .field("name", &self.name())
            .field("type", &self.type_())
            .field("flags", &self.flags())
            .finish()
    }
}
