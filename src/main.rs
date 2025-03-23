fn main() {

    pub struct Vector(f32, f32, f32);

    impl Vector{
        pub fn sub(&self, vector:Vector) -> Vector{
            Vector(
                self.0 - vector.0,
                self.1 - vector.1,
                self.2 - vector.2
            )
        }

        pub fn add(&self, vector:Vector) -> Vector{
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

        pub fn lerp(&self, vector:Vector, alpha: f32) -> Vector{
            self.scale(1.0-alpha).add(vector.scale(alpha))
        }


    }


    let x1 = Vector(1.0, 0.75, 0.0);
    let x2 = Vector(-1.0, 0.75, 0.0);
    let x3 = Vector(1.0, -0.75, 0.0);
    let x4= Vector(-1.0, -0.75, 0.0);

    let c = Vector(0.0,0.0,-1.0);

    let alpha = 0.3;
    let beta = 0.4;

    let t = x1.lerp(x2, alpha);
    let b = x3.lerp(x4, alpha);
    let p = t.lerp(b, beta);
    println!("meow{:?}", [p.0, p.1, p.2])

}



