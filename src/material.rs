use crate::ray::{HitRecord, Ray};
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = record.normal + Vec3::random_unit_vector();

        let scatter_direction = if scatter_direction.near_zero() {
            record.normal
        } else {
            scatter_direction
        };

        Some((Ray::new(record.point, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = mirror_reflect(ray.direction.unit(), record.normal);

        if reflected.dot(record.normal) > 0.0 {
            Some((Ray::new(record.point, reflected), self.albedo))
        } else {
            None
        }
    }
}

pub fn mirror_reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}
