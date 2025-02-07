// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Sphere;
use crate::Vec3;
use glib::translate::*;

impl Sphere {
    #[doc(alias = "graphene_sphere_init")]
    pub fn new(center: Option<&Point3D>, radius: f32) -> Sphere {
        assert_initialized_main_thread!();
        unsafe {
            let mut sph = Sphere::uninitialized();
            ffi::graphene_sphere_init(sph.to_glib_none_mut().0, center.to_glib_none().0, radius);
            sph
        }
    }

    #[doc(alias = "graphene_sphere_init_from_points")]
    #[doc(alias = "new_from_points")]
    pub fn from_points(points: &[Point3D], center: Option<&Point3D>) -> Sphere {
        assert_initialized_main_thread!();

        let n = points.len() as u32;

        unsafe {
            let mut sph = Sphere::uninitialized();
            ffi::graphene_sphere_init_from_points(
                sph.to_glib_none_mut().0,
                n,
                points.to_glib_none().0,
                center.to_glib_none().0,
            );
            sph
        }
    }

    #[doc(alias = "graphene_sphere_init_from_vectors")]
    #[doc(alias = "new_from_vectors")]
    pub fn from_vectors(vectors: &[Vec3], center: Option<&Point3D>) -> Sphere {
        assert_initialized_main_thread!();

        let n = vectors.len() as u32;

        unsafe {
            let mut sph = Sphere::uninitialized();
            ffi::graphene_sphere_init_from_vectors(
                sph.to_glib_none_mut().0,
                n,
                vectors.to_glib_none().0,
                center.to_glib_none().0,
            );
            sph
        }
    }
}
