//! Defines a standard plane piece of geometry

use std::num::Float;

use geometry::{Geometry, DifferentialGeometry};
use linalg::{Normal, Vector, Ray};


/// A plane centered at the origin spanning [-1, -1] to [1, 1] with a normal along [0, 0, 1]
#[derive(Copy)]
pub struct Plane;

impl Geometry for Plane {
    fn intersect(&self, ray: &mut Ray) -> Option<DifferentialGeometry> {
        // If the ray is perpindicular to the normal it can't intersect
        if Float::abs(ray.d.z) < 1e-8 {
            return None;
        }
        // Test for intersection against an infinite plane. Later we will
        // check that the hit found here is in the finite plane's extent
        let t = -ray.o.z / ray.d.z;
        if t < ray.min_t || t > ray.max_t {
            return None;
        }
        let p = ray.at(t);
        if p.x >= -1.0 && p.x <= 1.0 && p.y >= -1.0 && p.y <= 1.0 {
            ray.max_t = t;
            let n = Normal::new(0.0, 0.0, 1.0);
            let dp_du = Vector::new(2.0, 0.0, 0.0);
            let dp_dv = Vector::new(0.0, 2.0, 0.0);
            Some(DifferentialGeometry::new(&p, &n, &dp_du, &dp_dv, self))
        } else {
            None
        }
    }
}
