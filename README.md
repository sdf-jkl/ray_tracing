## 🚀 Motivation

I came across [@matklad](https://github.com/matklad)'s blog post where he mentions that one of the best ways to learn a programming language is by writing a **ray tracer** — so I decided to give it a shot.

He links to [*Build Your Own 3D Renderer* by Avik Das](https://avikdas.com/build-your-own-raytracer/), a guide aimed at people who aren’t super confident with the math side of things. It’s a great resource for looking up the necessary formulas. I initially planned to just skim it… but ended up following the entire guide.

---

## 🛠️ Process

### 📚 Project 1: Casting rays from the camera to the image plane

This project had 6 main steps:

1. **Clone the project repo**  
   I skipped this since I wanted to build everything from scratch.

2. **Represent a 3D vector**  
   Implemented a `Vector` struct with several methods (e.g., addition, subtraction, dot product).

3. **Represent the image plane and camera**  
   Defined 4 corner vectors for the image plane and a camera vector behind it.

4. **Determine where to cast rays**  
   Used linear interpolation to find points across the image plane.

5. **Represent a ray**  
   Modeled a ray by subtracting the camera position from each image plane point.

6. **Visualize the rays**  
   This was the trickiest step. I used the [image crate](https://docs.rs/image/latest/image/) and wrote a loop to color each pixel in a `256x192` image, generating a nice gradient based on ray direction.

**This was the result**:  
![Pretty gradient result](https://raw.githubusercontent.com/sdf-jkl/ray_tracing/baeded2ded6fe331a6b577c3ba40fadaea386828/output.png)

---

### 📚 Project 2: Rendering unshaded spheres

This one had 4 steps:

1. **Add spheres to the world**  
   Implemented a `Sphere` struct with two fields: `center` (a `Vector`) and `radius`.

2. **Add color to the spheres**  
   Added a third field — `color` (a tuple with three `f32` values).

3. **Perform ray-sphere intersection tests**  
   This was the challenging step. I got stuck for over an hour because of a bug in the `length` method I added to the `Vector` struct. Once fixed, I looped through all objects for each ray and picked the first one it intersected.

4. **Figure out which sphere a ray sees**  
   After getting the math right, I just had to return the color of the closest intersecting sphere.

**And the result was... this!**  
![This kinda looks like planetary alignment in front of the Sun](https://raw.githubusercontent.com/sdf-jkl/ray_tracing/807a6c77d669f2bc1fe66177e5e8f45aa7d44398/output.png)

---
