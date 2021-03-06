#![allow(dead_code)]

use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;  

pub struct Camera {
    pub origin: Point3, 
    pub lower_left_corner: Point3, 
    pub horizontal: Vec3, 
    pub vertical: Vec3, 
}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64, origin: Vec3) -> Self {
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0); 
        let vertical = Vec3::new(0.0, viewport_height, 0.0); 
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin, 
            horizontal, 
            vertical, 
            lower_left_corner, 
        }
    }

    pub fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0; 
        let viewport_height = 2.0; 
        let viewport_width = viewport_height * aspect_ratio; 
        let focal_length = 1.0; 

        let origin = Point3::new(0.0, 0.0, 0.0); 
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0); 
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length); 
        
        Self {
            origin, 
            horizontal, 
            vertical, 
            lower_left_corner, 
        }
    }

    pub fn ray_at(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin, 
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}