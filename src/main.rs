use ray_tracing::{
    ray_tracing_with_ssaa, Color, Frame, Light, Material, SamplePattern, Scene, Sphere, Vector,
};

//Setting the scene
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
        width: 256,  //256 1920
        height: 192, //192 192
        ambient_light: Color(0.1, 0.1, 0.1),
        lights: vec![light1, light2],
        spheres: vec![sphere1, sphere2, sphere3],
    };

    ray_tracing_with_ssaa(&scene, SamplePattern::Nine);
}
