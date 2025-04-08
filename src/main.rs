use image::{Rgb, RgbImage};

fn main() {

    pub struct Vector(f32, f32, f32);

    // Implemeting functions for the Vector struct
    impl Vector{
        pub fn sub(&self, vector:&Vector) -> Vector{
            Vector(
                self.0 - vector.0,
                self.1 - vector.1,
                self.2 - vector.2
            )
        }

        pub fn add(&self, vector:&Vector) -> Vector{
            Vector(
                self.0 + vector.0,
                self.1 + vector.1,
                self.2 + vector.2
            )
        }

        pub fn scale(&self, scalar: f32) -> Vector{
            Vector(
                self.0 * scalar,
                self.1 * scalar, 
                self.2 * scalar)
        }

        pub fn lerp(&self, vector:&Vector, alpha: f32) -> Vector{
            self.scale(1.0-alpha).add(&vector.scale(alpha))
        }

        pub fn to_rgb(&self) -> [u8; 3] {
            [
                ((self.0.clamp(-1.0, 1.0) * 0.5 + 0.5) * 255.0) as u8,
                ((self.1.clamp(-1.0, 1.0) * 0.5 + 0.5) * 255.0) as u8,
                // ((self.2.clamp(-1.0, 1.0) * 0.5 + 0.5) * 255.0) as u8,
                127
            ]
        }


    }

    // Image plain corners vectors
    let x1 = Vector(1.0, 0.75, 0.0); //Top right
    let x2 = Vector(-1.0, 0.75, 0.0); //Botton right
    let x3 = Vector(1.0, -0.75, 0.0); // Top left
    let x4= Vector(-1.0, -0.75, 0.0); //Bottom left

    // Camera vector
    let c = Vector(0.0,0.0,-1.0);

    let width = 256;
    let height = 192;

    let mut img = RgbImage::new(width, height);

    // Looping through each pixel in the 256x192 plain
    // and printing it's coordinates
    for x in 0..width {
        for y in 0..height {
            let alpha = x as f32/(width - 1) as f32;
            let beta = y as f32/(height -1) as f32;
            let t = x1.lerp(&x2, alpha);
            let b = x3.lerp(&x4, alpha);
            let p = t.lerp(&b, beta);
            let o = p.sub(&c);
            
            let pixel = Rgb(o.to_rgb());
            img.put_pixel(x, y, pixel);
        }
    }

    img.save("output.png").expect("Failed to save image")
}



