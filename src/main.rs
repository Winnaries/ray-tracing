use ray::{Hittable, HittableList, Ray};
use sphere::Sphere;
use std::rc::Rc;
use std::f64::INFINITY; 
use vec3::{Color, Point3, Vec3};

mod ray;
mod sphere;
mod vec3;

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    if let Some(record) = world.hit(ray, 0.0, INFINITY) {
        return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as u64;
    let image_height = (image_width as f64 / aspect_ratio).round() as u64;

    let mut world = HittableList::new();
    let sphere_a: Box<dyn Hittable> = Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sphere_b: Box<dyn Hittable> = Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let sphere_a = Rc::new(sphere_a);
    let sphere_b = Rc::new(sphere_b);

    world.add(sphere_a);
    world.add(sphere_b);

    let viewport_height = 2.0 as f64;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = Ray {
                origin,
                direction: lower_left_corner + horizontal * u + vertical * v - origin,
            };

            let pixel = ray_color(ray, &world);
            println!("{}", pixel.rgb());
        }
    }

    eprintln!();
    eprintln!("Done.");
}