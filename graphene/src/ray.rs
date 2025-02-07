// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Ray;
use crate::Vec3;
use glib::translate::*;

impl Ray {
    #[doc(alias = "graphene_ray_init")]
    pub fn new(origin: Option<&Point3D>, direction: Option<&Vec3>) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let mut ray = Ray::uninitialized();
            ffi::graphene_ray_init(
                ray.to_glib_none_mut().0,
                origin.to_glib_none().0,
                direction.to_glib_none().0,
            );
            ray
        }
    }

    #[doc(alias = "graphene_ray_init_from_ray")]
    #[doc(alias = "new_from_ray")]
    pub fn from_ray(src: &Ray) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let mut ray = Ray::uninitialized();
            ffi::graphene_ray_init_from_ray(ray.to_glib_none_mut().0, src.to_glib_none().0);
            ray
        }
    }

    #[doc(alias = "graphene_ray_init_from_vec3")]
    #[doc(alias = "new_from_vec3")]
    pub fn from_vec3(origin: Option<&Vec3>, direction: Option<&Vec3>) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let mut ray = Ray::uninitialized();
            ffi::graphene_ray_init_from_vec3(
                ray.to_glib_none_mut().0,
                origin.to_glib_none().0,
                direction.to_glib_none().0,
            );
            ray
        }
    }
}
