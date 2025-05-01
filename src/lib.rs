use core::f32;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector(pub f32, pub f32, pub f32);

// Implemeting functions for the Vector struct
impl Add for Vector {
    type Output = Vector;

    fn add(self, vector: Vector) -> Vector {
        Vector(self.0 + vector.0, self.1 + vector.1, self.2 + vector.2)
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, vector: &Vector) -> Vector {
        *self + *vector
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, vector: Vector) -> Vector {
        Vector(self.0 - vector.0, self.1 - vector.1, self.2 - vector.2)
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, vector: &Vector) -> Vector {
        *self - *vector
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        *self * scalar
    }
}

impl Mul for Vector {
    type Output = f32;

    fn mul(self, vector: Vector) -> f32 {
        self.0 * vector.0 + self.1 * vector.1 + self.2 * vector.2
    }
}

impl Mul for &Vector {
    type Output = f32;

    fn mul(self, vector: &Vector) -> f32 {
        *self * *vector
    }
}

impl Vector {
    pub fn norm(&self) -> Vector {
        let len = self.len();
        if len == 0.0 {
            panic!("Tried to normalize a zero-length vector");
        }
        self.mul(1.0 / len)
    }

    pub fn lerp(&self, vector: &Vector, alpha: f32) -> Vector {
        // Linear Extrapolation
        self * (1.0 - alpha) + (vector * alpha)
    }

    pub fn len(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [
            ((self.0.clamp(-1.0, 1.0) * 0.5 + 0.5) * 255.0) as u8,
            ((self.1.clamp(-1.0, 1.0) * 0.5 + 0.5) * 255.0) as u8,
            // ((self.2.clamp(-1.0, 1.0) * 0.5 + 0.5) * 255.0) as u8,
            127,
        ]
    }
}

#[derive(Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32);

impl Add for Color {
    type Output = Color;

    fn add(self, color: Color) -> Color {
        Color(self.0 + color.0, self.1 + color.1, self.2 + color.2)
    }
}

impl Add for &Color {
    type Output = Color;

    fn add(self, color: &Color) -> Color {
        *self + *color
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color(self.0 * color.0, self.1 * color.1, self.2 * color.2)
    }
}

impl Mul for &Color {
    type Output = Color;

    fn mul(self, color: &Color) -> Color {
        *self * *color
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl Mul<f32> for &Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        *self * scalar
    }
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: Color, // needs to be 0.0..1.0
    pub material: Material,
}

impl Sphere {
    pub fn intersection_point(&self, origin: &Vector, d: &Vector, t: f32) -> Vector {
        *origin + (d * t)
    }
    pub fn surf_normal(&self, p: &Vector) -> Vector {
        p.sub(&self.center).norm()
    }
}

pub fn intersection_test(direction: &Vector, sphere: &Sphere, origin: &Vector) -> f32 {
    let c_prime = origin.sub(&sphere.center);

    let a = direction * direction;
    let b = c_prime * *direction * 2.0;
    let c = c_prime.len().powi(2) - sphere.radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;

    match discriminant.is_sign_positive() {
        false => return 0.0,
        true => discriminant,
    };

    let t_plus = (-b + discriminant.sqrt()) / (2.0 * a);
    let t_plus = match t_plus > 0.0 {
        true => t_plus,
        false => f32::NAN,
    };
    let t_minus = (-b - discriminant.sqrt()) / (2.0 * a);
    let t_minus = match t_minus > 0.0 {
        true => t_minus,
        false => f32::NAN,
    };
    let t = t_plus.min(t_minus);

    match t.is_nan() {
        true => 0.0,
        false => t,
    }
}

pub struct Light {
    pub location: Vector,
    pub diffuse_int: Color,  // needs to be 0.0..1.0
    pub specular_int: Color, // needs to be 0.0..1.0
}

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient_k: Color,
    pub diffuse_k: Color,
    pub specular_k: Color,
    pub reflectivity_k: Color, //if increse refl color decrease the same diffuse color
    pub shininess: i32,
}

pub struct Scene {
    pub frame: Frame,
    pub camera: Vector,
    pub width: u32,
    pub height: u32,
    pub ambient_light: Color,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

pub struct Frame {
    pub x1: Vector, //Top right
    pub x2: Vector, //Top left
    pub x3: Vector, // Bottom right
    pub x4: Vector, //Bottom left
}

pub fn ray_tracer(scene: &Scene, d: Vector) -> Color {
    let mut intersections = Vec::new();

    for sphere in &scene.spheres {
        let t = intersection_test(&d, &sphere, &scene.camera);

        if t > 0.0 {
            intersections.push((t, sphere));
        }
    }
    if let Some((t, closest_sphere)) = intersections
        .iter()
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    {
        let p_inter = closest_sphere.intersection_point(&scene.camera, &d, *t);

        let surf_normal = closest_sphere.surf_normal(&p_inter);

        let ambient_term = scene.ambient_light * closest_sphere.material.ambient_k;

        let mut color = closest_sphere.color.clone();

        color = color + ambient_term;

        for light in &scene.lights {
            let light_vector = (light.location - p_inter).norm();

            let shadow_ray = light.location - p_inter;

            let mut in_shadow = false;

            for shadow_sphere in &scene.spheres {
                if shadow_sphere.center == closest_sphere.center {
                    continue;
                }

                let shadow_t = intersection_test(&shadow_ray, &shadow_sphere, &p_inter);

                if shadow_t > 0.0 && shadow_t < 1.0 {
                    in_shadow = true;
                    break;
                }
            }
            if !in_shadow {
                let dot_prod = surf_normal * light_vector;
                if dot_prod > 0.0 {
                    let diffuse_comp =
                        light.diffuse_int * closest_sphere.material.diffuse_k * dot_prod;

                    let refl_vector = surf_normal * dot_prod * 2.0 - light_vector;

                    let view_vector = (scene.camera - p_inter).norm();

                    let specular_comp = closest_sphere.material.specular_k
                        * light.specular_int
                        * (view_vector * refl_vector).powi(closest_sphere.material.shininess);

                    color = color + diffuse_comp + specular_comp;

                    
                }
            }
        }
        return color;
    } else {
        Color(0.0, 0.0, 0.0)
    }
}