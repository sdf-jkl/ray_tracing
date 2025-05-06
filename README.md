## 🚀 Motivation  
![High-res image](https://raw.githubusercontent.com/sdf-jkl/ray_tracing/2b27e1cff8134c0b1e15c6e502d20a3310c7149f/output.png)

I came across [@matklad](https://github.com/matklad)'s blog post where he mentions that one of the best ways to learn a programming language is by writing a **ray tracer** - so I decided to give it a shot.

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
   Added a third field - `color` (a tuple with three `f32` values).

3. **Perform ray-sphere intersection tests**  
   This was the challenging step. I got stuck for over an hour because of a bug in the `length` method I added to the `Vector` struct. Once fixed, I looped through all objects for each ray and picked the first one it intersected.

4. **Figure out which sphere a ray sees**  
   After getting the math right, I just had to return the color of the closest intersecting sphere.

**And the result was... this!**  
![This kinda looks like planetary alignment in front of the Sun](https://raw.githubusercontent.com/sdf-jkl/ray_tracing/807a6c77d669f2bc1fe66177e5e8f45aa7d44398/output.png)

---

### 📚 Project 3: Rendering shaded spheres

1. **Add lights to the world**  
   Implemented a `Lights` struct with three fields: `location`, `diffuse_int`, and `specular_int`, and added some lights to the scene.

2. **Add materials to spheres**  
   Added a `material` field to the `Sphere` struct with multiple light-related properties.

3. **Calculate the point of intersection and surface normal**  
   Wrote two lines to calculate `p_intersect` and `surf_normal`.

4. **Calculate the ambient term**  
   Straightforward implementation.

5. **Calculate the diffuse term**  
   Involved a few additional formulas.

6. **Calculate the specular term**  
   Slightly larger formulas, but nothing too complex.

7. **Clamp the resulting color**  
   Combined all the lighting terms with the sphere color and clamped the result to the range [0, 1].


   **And this was the result:**  
   ![Spheres with lights](https://raw.githubusercontent.com/sdf-jkl/ray_tracing/c0e265e777d78c8353c3860e3ef49cc71b9cf4ec/output.png)

---

### 📚 Project 4: Rendering shadows

1. **Cast shadow rays at each intersection point**  
   Casted `shadow_ray`s for every light from every intersection with a sphere

2. **Determine if a sphere is in shadow from a light**  
   Did another for loop through spheres to see if the `shadow_ray` interesects one and is casting a shadow

   **Whew, this was short, but had some trouble with the second step:**  
   ![Spheres with shadows](https://raw.githubusercontent.com/sdf-jkl/ray_tracing/c5a73e01a9376324bc13ac20ce2b63460ecbf650/output.png)

---

### 📚 Project 5: Rendering reflections

1. **Model the reflectivity of a material**  
   Added reflectivity constant to the `material` struct

2. **Determine if a sphere is in shadow from a light**  
   Packed the color creating part into `ray_tracer` function that takes a `scene`, `ray`, `origin` vector and now added recursion `depth` and made the function recursive

3. **Calculate the reflectance vector**
  Did a little formula to calculate the reflectance vector

5. **Recursively apply the ray tracing algorithm**
   Ran the algorithm recursively for every ray coming from the camera

   **The recursion part took a little effort, but not too crazy:**  
   ![Spheres with reflections](https://raw.githubusercontent.com/sdf-jkl/ray_tracing/1ed9cd44effeff63e60f14b9f4579b253870aed2/output.png)

---

### 📚 Project 6: Anti-aliasing

1. **Calculate multiple points within one pixel boundary**  
   Created a `SamplePattern` enum with three common anti-aliasing parameters and tuples including the logic to split the pixel into multiple parts

2. **Average the results of the samples**  
   Ran the `ray_tracer` for every pixel part, summed the resuling `Color` and divided it by the number of points to get the average 

   **The spheres are now more smooth with 9 SSAA:**  
   ![Spheres with SSAA](https://raw.githubusercontent.com/sdf-jkl/ray_tracing/33f9f12ffc08fc6905d10c844a6a0367b7d9be70/output.png)

---

## 🚀 Performance Optimization  
Adding Anti-Aliasing was the last project in the guide, so I am free sailing now.

The last three projects added a lot of iterations and recursive calls making the algorithm much slower now.

But how much slower?

At the original resolution of **256×192**, I didn’t notice any real difference - Rust is too fast.  
To stress test the renderer, I increased the resolution to **2048×1536** (still a 4:3 aspect ratio) and ran a benchmark using [`hyperfine`](https://github.com/sharkdp/hyperfine):

```bash
hyperfine 'cargo run'  
Benchmark 1: cargo run  
  Time (mean ± σ):     27.503 s ±  0.183 s    [User: 13.766 s, System: 0.257 s]  
  Range (min … max):   27.360 s … 27.995 s    10 runs
```

Need to fix this now...

