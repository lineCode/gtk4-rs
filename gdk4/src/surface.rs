// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Surface;
use glib::object::IsA;
use glib::translate::*;

pub trait SurfaceExtManual: 'static {
    fn create_similar_surface(
        &self,
        content: cairo::Content,
        width: i32,
        height: i32,
    ) -> Option<cairo::Surface>;

    fn translate_coordinates(&self, to: &Surface, x: f64, y: f64) -> bool;
}

impl<O: IsA<Surface>> SurfaceExtManual for O {
    fn create_similar_surface(
        &self,
        content: cairo::Content,
        width: i32,
        height: i32,
    ) -> Option<cairo::Surface> {
        unsafe {
            from_glib_full(ffi::gdk_surface_create_similar_surface(
                self.as_ref().to_glib_none().0,
                content.into(),
                width,
                height,
            ))
        }
    }

    fn translate_coordinates(&self, to: &Surface, mut x: f64, mut y: f64) -> bool {
        unsafe {
            from_glib(ffi::gdk_surface_translate_coordinates(
                self.as_ref().to_glib_none().0,
                to.to_glib_none().0,
                &mut x,
                &mut y,
            ))
        }
    }
}
