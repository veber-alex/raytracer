#![warn(rust_2018_idioms)]

use crate::{hittable_list::HittableList, sphere::Sphere, vec3::Point3};
use bvh::BvhNode;
use camera::Camera;
use color::Color;
use material::{Dielectric, Lambertian, Metal};
use rtweekend::{random_double, random_double_min_max};
use texture::{CheckerTexture, ImageTexture};
use vec3::Vec3;

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtw_image;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

fn random_spheres() {
    let mut world = HittableList::new();

    let checker =
        CheckerTexture::from_solid(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Lambertian::new(checker),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::from_color(albedo);
                    let center2 = center + Vec3::new(0., random_double_min_max(0., 0.5), 0.);
                    world.add(Sphere::moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_min_max(0.5, 1.);
                    let fuzz = random_double_min_max(0., 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0., 1., 0.), 1.0, material1));

    let material2 = Lambertian::from_color(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.);
    world.add(Sphere::new(Point3::new(4.0, 1., 0.), 1., material3));

    world = HittableList::from_hittable(BvhNode::from_list(world));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.;

    cam.render(&world);
}

fn two_spheres() {
    let mut world = HittableList::new();

    let checker =
        CheckerTexture::from_solid(0.8, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    world.add(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Lambertian::new(checker.clone()),
    ));

    world.add(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Lambertian::new(checker),
    ));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.;

    cam.render(&world);
}

fn earth() {
    let earth_texture =
        ImageTexture::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/earthmap.jpg",));
    let earth_surface = Lambertian::new(earth_texture);
    let globe = Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface);

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Point3::new(0., 0., 12.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.;

    cam.render(&HittableList::from_hittable(globe))
}

fn main() {
    match 3 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        _ => {}
    }
}
