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
        center: Vector(0.2, -0.5, 2.0),
        radius: 1.0,
        color: Color(0.7, 0.0, 0.0), // Red
        material: Material {
            ambient_k: Color(0.2, 0.2, 0.2),
            diffuse_k: Color(0.5, 0.7, 0.6),
            specular_k: Color(0.8, 0.7, 0.9),
            shininess: 20,
        },
    };

    let sphere2 = Sphere {
        center: Vector(-2.0, 0.5, 4.0),
        radius: 1.5,
        color: Color(0.0, 0.2, 0.0), // Green
        material: Material {
            ambient_k: Color(0.2, 0.1, 0.1),
            diffuse_k: Color(0.5, 0.8, 0.6),
            specular_k: Color(0.5, 0.7, 0.6),
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
            shininess: 20
        },
    };

    let spheres = vec![sphere1, sphere2, sphere3];

    let ambient_light = Color(0.1, 0.1, 0.1);

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

                let ambient_term = ambient_light.prod(&closest_sphere.material.ambient_k);

                let mut color = closest_sphere.color.clone();

                color = color.add(&ambient_term);

                for light in &lights {
                    let light_vector = light.location.sub(&p_inter).norm();

                    let dot_prod = surf_normal.dot_prod(&light_vector);
                    if dot_prod > 0.0 {
                        let diffuse_comp = light
                            .diffuse_int
                            .prod(&closest_sphere.material.diffuse_k)
                            .scale(dot_prod);

                        let refl_vector = surf_normal.scale(2.0 * dot_prod).sub(&light_vector);

                        let view_vector = c.sub(&p_inter).norm();

                        let specular_comp = closest_sphere
                            .material
                            .specular_k
                            .prod(&light.specular_int)
                            .scale(
                                view_vector
                                    .dot_prod(&refl_vector)
                                    .powi(closest_sphere.material.shininess),
                            );

                        color = color.add(&diffuse_comp).add(&specular_comp)
                    }
                }
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
            } else {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }

    img.save("output.png").expect("Failed to save image")
}
