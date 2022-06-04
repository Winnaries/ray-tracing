use camera::Camera;
use random::random_double;
use ray::{Hittable, HittableList, Ray};
use sphere::Sphere;
use std::f64::INFINITY;
use std::rc::Rc;
use vec3::{Color, Point3, Vec3};

mod camera;
mod random;
mod ray;
mod sphere;
mod vec3;

fn ray_color(ray: Ray, world: &HittableList, depth: u64) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0); 
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        let target = hit.point + hit.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(Ray::new(hit.point, target - hit.point), world, depth - 1);
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as u64;
    let image_height = (image_width as f64 / aspect_ratio).round() as u64;
    let sample_per_pixel = 100 as u64;
    let max_depth = 50; 

    let mut world = HittableList::new();
    let sphere_a: Box<dyn Hittable> = Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sphere_b: Box<dyn Hittable> = Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let sphere_a = Rc::new(sphere_a);
    let sphere_b = Rc::new(sphere_b);

    world.add(sphere_a);
    world.add(sphere_b);

    let camera = Camera::default(); 

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        for i in 0..image_width {
            let mut sampled_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..sample_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                let ray = camera.ray_at(u, v);
                let color = ray_color(ray, &world, max_depth);
                sampled_color += color;
            }

            println!("{}", sampled_color.rgb(sample_per_pixel));
        }
    }

    eprintln!();
    eprintln!("Done.");
}
