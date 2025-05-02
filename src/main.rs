use image::{Rgb, RgbImage};
use ray_tracing::{Color, Frame, Light, Material, Scene, Sphere, Vector, ray_tracer};

fn main() {
    let sphere1 = Sphere {
        center: Vector(0.2, -0.5, 2.0),
        radius: 1.0,
        color: Color(0.7, 0.0, 0.0), // Red
        material: Material {
            ambient_k: Color(0.2, 0.2, 0.2),
            diffuse_k: Color(0.3, 0.2, 0.1),
            specular_k: Color(0.8, 0.7, 0.9),
            reflectivity_k: Color(0.5, 0.7, 0.8),
            shininess: 20,
        },
    };

    let sphere2 = Sphere {
        center: Vector(-2.0, 0.5, 4.0),
        radius: 1.5,
        color: Color(0.0, 0.2, 0.0), // Green
        material: Material {
            ambient_k: Color(0.2, 0.1, 0.1),
            diffuse_k: Color(0.3, 0.2, 0.1),
            specular_k: Color(0.5, 0.7, 0.6),
            reflectivity_k: Color(0.5, 0.7, 0.8),
            shininess: 20,
        },
    };

    let sphere3 = Sphere {
        center: Vector(2.0, 0.6, 3.0),
        radius: 0.9,
        color: Color(0.0, 0.0, 0.8), // Blue
        material: Material {
            ambient_k: Color(0.0, 0.2, 0.1),
            diffuse_k: Color(0.6, 0.9, 0.7),
            specular_k: Color(0.8, 0.9, 0.7),
            reflectivity_k: Color(0.3, 0.1, 0.2),
            shininess: 20,
        },
    };

    let light1 = Light {
        location: Vector(-2.0, -1.0, 1.0),
        diffuse_int: Color(0.2, 0.3, 0.3),
        specular_int: Color(0.1, 0.5, 0.5),
    };

    let light2 = Light {
        location: Vector(1.5, 1.0, 1.0),
        diffuse_int: Color(0.4, 0.7, 0.3),
        specular_int: Color(0.2, 0.1, 0.2),
    };

    let scene = Scene {
        frame: Frame {
            x1: Vector(1.0, 0.75, 0.0),
            x2: Vector(-1.0, 0.75, 0.0),
            x3: Vector(1.0, -0.75, 0.0),
            x4: Vector(-1.0, -0.75, 0.0),
        },
        camera: Vector(0.0, 0.0, -1.0),
        width: 256,
        height: 192,
        ambient_light: Color(0.1, 0.1, 0.1),
        lights: vec![light1, light2],
        spheres: vec![sphere1, sphere2, sphere3],
    };

    let mut img = RgbImage::new(scene.width, scene.height);

    // Looping through each pixel in the 256x192 plain
    // and printing it's coordinates

    for x in 0..scene.width {
        for y in 0..scene.height {
            let alpha = x as f32 / (scene.width - 1) as f32;
            let beta = y as f32 / (scene.height - 1) as f32;
            let t = scene.frame.x2.lerp(&scene.frame.x1, alpha);
            let b = scene.frame.x4.lerp(&scene.frame.x3, alpha);
            let p = t.lerp(&b, beta);
            let direction_ray = (p - scene.camera).norm();

            let mut color = ray_tracer(&scene, direction_ray, &scene.camera, 3);

            color = Color(
                color.0.clamp(0.0, 1.0),
                color.1.clamp(0.0, 1.0),
                color.2.clamp(0.0, 1.0),
            );

            img.put_pixel(
                x,
                y,
                Rgb([
                    (color.0 * 255.0) as u8,
                    (color.1 * 255.0) as u8,
                    (color.2 * 255.0) as u8,
                ]),
            );
        }
    }

    img.save("output.png").expect("Failed to save image")
}
