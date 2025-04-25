use core::f32;

pub struct Vector(pub f32, pub f32, pub f32);

// Implemeting functions for the Vector struct
impl Vector {
    pub fn sub(&self, vector: &Vector) -> Vector {
        Vector(self.0 - vector.0, self.1 - vector.1, self.2 - vector.2)
    }

    pub fn add(&self, vector: &Vector) -> Vector {
        Vector(self.0 + vector.0, self.1 + vector.1, self.2 + vector.2)
    }

    pub fn scale(&self, scalar: f32) -> Vector {
        Vector(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }

    pub fn norm(&self) -> Vector {
        let len = self.len();
        if len == 0.0 {
            panic!("Tried to normalize a zero-length vector");
        }
        self.scale(1.0 / len)
    }

    pub fn dot_prod(&self, vector: &Vector) -> f32 {
        self.0 * vector.0 + self.1 * vector.1 + self.2 * vector.2
    }

    pub fn lerp(&self, vector: &Vector, alpha: f32) -> Vector {
        // Linear Extrapolation
        self.scale(1.0 - alpha).add(&vector.scale(alpha))
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

pub struct Color(pub f32, pub f32, pub f32);

pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: Color, // needs to be 0.0..1.0
    pub material: Material
}

impl Sphere{
    pub fn intersection_point(&self, origin: &Vector, d: &Vector, t: f32) -> Vector{
        origin.add(&d.scale(t))
    }
    pub fn surf_normal(&self, p: &Vector) -> Vector {
        p.sub(&self.center).norm()
    }
}

pub fn intersection_test(direction: &Vector, sphere: &Sphere, origin: &Vector) -> f32 {
    let c_prime = origin.sub(&sphere.center);

    let a = direction.dot_prod(&direction);
    let b = 2.0 * c_prime.dot_prod(&direction);
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

pub struct Material {
    pub ambient_k: Color,
    pub diffuse_k: f32,
    pub specular_k: f32,
    pub shininess: i32
}