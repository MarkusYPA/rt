
# 🛠️ Step-by-Step Plan for the Ray Tracer in Rust

---

## **Stage 1 – Setup and PPM Output**

**Goal:** Make sure we can generate and view an image file.

1. Initialize a new Rust project with `cargo new rt`.
2. Implement a function to generate a `.ppm` image:

   * Write the P3 header (`P3`, width, height, max color).
   * Fill the pixels with simple gradients (e.g., red → blue).
3. Test: Run `cargo run > test.ppm` and open the file in an image viewer (GIMP, IrfanView, or online PPM viewer).

✅ **Checkpoint:** You can produce a colored gradient `.ppm` image.

---

## **Stage 2 – Ray and Camera**

**Goal:** Shoot rays from a camera into the scene.

1. Define a `Vec3` struct for 3D math (with dot, cross, normalize, etc.).
2. Define a `Ray` struct with origin and direction.
3. Implement a simple `Camera` that maps pixels (x,y) to rays.

   * Start with a fixed camera at `(0,0,0)` looking into the scene.

✅ **Checkpoint:** For each pixel, output a color based on the ray direction (e.g., sky gradient: blue above, white below).

---

## **Stage 3 – Sphere Intersection**

**Goal:** Draw a sphere by detecting ray hits.

1. Implement `hit_sphere(center, radius, ray)` returning true/false.

   * Formula: Solve quadratic for intersection.
2. For pixels where a ray hits the sphere, color it red.
3. For pixels where no hit, use background gradient.

✅ **Checkpoint:** A red circle (sphere) appears in the image.

---

## **Stage 4 – Normals and Shading**

**Goal:** Add lighting/shadows.

1. Compute surface normal at the hit point.
2. Implement a simple diffuse shading model:

   * Color = `object_color * max(0, normal · light_direction)`
3. Add one directional light (like the sun).

✅ **Checkpoint:** Sphere now has light shading (one side bright, one side darker).

---

## **Stage 5 – Multiple Objects**

**Goal:** Support multiple shapes.

1. Create a `trait Hittable` with a `hit(ray)` method.
2. Implement `Sphere` as the first `Hittable`.
3. Add a `World` struct containing a list of objects.
4. Extend shading to choose the closest hit.

✅ **Checkpoint:** Scene can render multiple spheres.

---

## **Stage 6 – More Shapes**

**Goal:** Implement the required 4 objects.

1. Add **Plane** (infinite plane with normal and point).
2. Add **Cube** (can use slabs method for intersection).
3. Add **Cylinder** (intersection = quadratic in x/z).
4. Test each object separately in small scenes.

✅ **Checkpoint:** Each object type renders individually.

---

## **Stage 7 – Scene Setup**

**Goal:** Place and move objects.

1. Add ability to set object position (via struct fields).
2. Add simple material struct (color, brightness).
3. Add ability to move camera position + direction.

✅ **Checkpoint:** Can render sphere at `(1,1,1)` and move camera.

---

## **Stage 8 – Shadows**

**Goal:** Rays must check for blocked light.

1. For each hit point, cast a “shadow ray” toward the light.
2. If another object is hit first → pixel is in shadow.
3. Darken color when in shadow.

✅ **Checkpoint:** Sphere casts shadow on plane.

---

## **Stage 9 – Final Scenes (Required 4)**

1. Scene 1: Sphere only.
2. Scene 2: Plane + cube, with cube dimmer.
3. Scene 3: Sphere + cube + cylinder + plane.
4. Scene 4: Same as scene 3, but different camera position.

✅ **Checkpoint:** Each scene renders as 800×600 `.ppm`.

---

## **Stage 10 – Documentation**

**Goal:** Write clear markdown guide.

* Explain how to run program.
* Show how to add objects (`Sphere::new(...)`).
* Show how to adjust brightness.
* Show how to move camera.
* Provide code snippets.

✅ **Checkpoint:** Auditor can run `cargo run > out.ppm` and follow docs.

---

# 🎯 Summary of Testable Milestones

1. Generate `.ppm` gradient.
2. Ray → sky gradient.
3. Single red sphere.
4. Shaded sphere.
5. Multiple objects (spheres).
6. All 4 shapes (sphere, cube, plane, cylinder).
7. Camera + object positioning.
8. Shadows.
9. Four required final scenes.
10. Documentation.
