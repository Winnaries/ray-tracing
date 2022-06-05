use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.lengthsq();
        let half_b = oc.dot(ray.direction);
        let c = oc.lengthsq() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let t = (-half_b - sqrtd) / a;
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let (front_face, normal) = HitRecord::face_normal(ray, outward_normal);

        Some(HitRecord {
            t,
            point,
            normal,
            front_face,
            material: self.material.clone(),
        })
    }
}
