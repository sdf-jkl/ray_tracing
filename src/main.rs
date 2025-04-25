use image::{Rgb, RgbImage};
use ray_tracing::{intersection_test, Color, Light, Material, Sphere, Vector};

fn main() {
    // Image plain corners vectors
    let x1 = Vector(1.0, 0.75, 0.0); //Top right
    let x2 = Vector(-1.0, 0.75, 0.0); //Top left
    let x3 = Vector(1.0, -0.75, 0.0); // Bottom right
    let x4 = Vector(-1.0, -0.75, 0.0); //Bottom left

    // Camera vector
    let c = Vector(0.0, 0.0, -1.0);

    // Plain dimensions in pixels
    let width = 256;
    let height = 192;

    let sphere1 = Sphere {
        center: Vector(0.2, -0.2, 2.0),
        radius: 1.0,
        color: Color(1.0, 0.0, 0.0), // Red
        material: Material{
            ambient_k: Color(0.2, 0.2, 0.2),
            diffuse_k: 0.5,
            specular_k: 0.3,
            shininess: 3
        }
    };

    let sphere2 = Sphere {
        center: Vector(-0.9, 0.5, 4.0),
        radius: 2.0,
        color: Color(0.0, 1.0, 0.0), // Green
        material: Material{
            ambient_k: Color(0.5, 0.3, 0.0),
            diffuse_k: 0.2,
            specular_k: 0.1,
            shininess: 2
        }
    };

    let sphere3 = Sphere {
        center: Vector(-0.75, 0.6, 3.0),
        radius: 1.2,
        color: Color(0.0, 0.0, 1.0), // Blue
        material: Material{
            ambient_k: Color(0.0, 0.9, 0.7),
            diffuse_k: 0.8,
            specular_k: 0.9,
            shininess: 6
        }
    };

    let spheres = vec![sphere1, sphere2, sphere3];

    let ambiant_light = Color(1.0, 1.0, 0.0);

    let light1 = Light{
        location: Vector(-1.0, 0.0, 2.0),
        diffuse_int: Color(0.2, 0.3, 0.3),
        specular_int: Color(0.1, 0.5, 0.5)
    };

    let light2 = Light{
        location: Vector(-0.7, 1.0, 3.0),
        diffuse_int: Color(0.4, 0.7, 0.3),
        specular_int: Color(0.5, 0.1, 0.8)
    };

    let lights = vec![light1, light2];

    let mut img = RgbImage::new(width, height);

    // Looping through each pixel in the 256x192 plain
    // and printing it's coordinates
    for x in 0..width {
        for y in 0..height {
            let alpha = x as f32 / (width - 1) as f32;
            let beta = y as f32 / (height - 1) as f32;
            let t = x2.lerp(&x1, alpha);
            let b = x4.lerp(&x3, alpha);
            let p = t.lerp(&b, beta);
            let d = p.sub(&c).norm();

            let mut intersections = Vec::new();

            for sphere in &spheres {
                let t = intersection_test(&d, &sphere, &c);

                if t > 0.0 {
                    intersections.push((t, sphere));
                }
            }
            if let Some((t, closest_sphere)) = intersections
                .iter()
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            {   
                let p_inter = closest_sphere.intersection_point(&c, &d, *t);

                let surf_normal = closest_sphere.surf_normal(&p_inter);

                for light in &lights {

                }


                let color = &closest_sphere.color;
                img.put_pixel(
                    x,
                    y,
                    Rgb([
                        (color.0 * 255.0) as u8,
                        (color.1 * 255.0) as u8,
                        (color.2 * 255.0) as u8,
                    ]),
                );
            } else {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }

    img.save("output.png").expect("Failed to save image")
}
