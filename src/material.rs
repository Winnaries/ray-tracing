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
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = mirror_reflect(ray.direction.unit(), record.normal);

        if reflected.dot(record.normal) > 0.0 {
            Some((
                Ray::new(
                    record.point,
                    reflected + self.fuzz * Vec3::random_within_unit_sphere(),
                ),
                self.albedo,
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.unit();
        let refracted = snell_refract(unit_direction, record.normal, refraction_ratio);

        Some((Ray::new(record.point, refracted), Color::new(1.0, 1.0, 1.0)))
    }
}

pub fn mirror_reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn snell_refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = n.dot(-uv).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.lengthsq()).abs().sqrt() * n;
    r_out_parallel + r_out_perp
}
