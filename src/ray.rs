#![allow(dead_code)]

use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn face_normal(ray: Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }
}

pub struct HittableList {
    pub objects: Vec<Rc<Box<dyn Hittable>>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![], 
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear(); 
    }

    pub fn add(&mut self, object: Rc<Box<dyn Hittable>>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max; 
        let mut temp_record: Option<HitRecord> = None; 

        for o in &self.objects {
            if let Some(record) = o.hit(ray, t_min, closest) {
                closest = record.t;
                temp_record = Some(record); 
            }
        }

        temp_record
    }
}