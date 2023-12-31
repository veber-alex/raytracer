use enum_dispatch::enum_dispatch;

use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    rtweekend::random_double,
    texture::{AnyTexture, SolidColor, Texture},
    vec3::Vec3,
};

#[enum_dispatch]
pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)>;
}

#[enum_dispatch(Material)]
#[derive(Clone)]
pub enum AnyMaterial {
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: AnyTexture,
}

impl Lambertian {
    pub fn new(a: impl Into<AnyTexture>) -> Self {
        Self { albedo: a.into() }
    }

    pub fn from_color(a: Color) -> Self {
        Self::new(SolidColor::new(a))
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);

        Some((attenuation, scattered))
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        Self {
            albedo,
            fuzz: if f < 1. { f } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().unit_vector().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_unit_vector(),
            r_in.time(),
        );
        let attenuation = self.albedo;

        if scattered.direction().dot(rec.normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    // Index of Refraction
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_double() {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction, r_in.time());

        Some((attenuation, scattered))
    }
}
