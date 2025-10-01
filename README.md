# Rust Ray Tracer

This is a simple ray tracer written in Rust. It can render scenes with spheres, cubes, planes, and cylinders.

## How to Run

To build the project, run the following command:

```bash
cargo build
```

To render a scene, run the following command:

```bash
cargo run -- [SCENE_NUMBER] > output.ppm
```

Where `[SCENE_NUMBER]` is a number from 1 to 4, corresponding to the four different scenes. If no scene number is provided, scene 1 will be rendered by default.

The output will be a `.ppm` image file.

### Scenes

- **Scene 1:** A single sphere.
- **Scene 2:** A flat plane and a cube.
- **Scene 3:** One of each of all the objects (one cube, one sphere, one cylinder and one flat plane).
- **Scene 4:** The same as scene 3, but from a different camera position.

## How to Add Objects

To add objects to a scene, you need to modify the `scene` functions in `src/main.rs`.

### Creating a Sphere

To create a sphere, you can use the `Sphere::new` function:

```rust
let sphere = Sphere::new(center, radius, material);
```

- `center`: A `Vec3` representing the center of the sphere.
- `radius`: A `f64` representing the radius of the sphere.
- `material`: A `Material` struct representing the color of the sphere.

### Creating a Plane

To create a plane, you can use the `Plane::new` function:

```rust
let plane = Plane::new(point, normal, material);
```

- `point`: A `Vec3` representing a point on the plane.
- `normal`: A `Vec3` representing the normal of the plane.
- `material`: A `Material` struct representing the color of the plane.

### Creating a Cube

To create a cube, you can use the `Cube::new` function:

```rust
let cube = Cube::new(min, max, material);
```

- `min`: A `Vec3` representing the minimum corner of the cube.
- `max`: A `Vec3` representing the maximum corner of the cube.
- `material`: A `Material` struct representing the color of the cube.

### Creating a Cylinder

To create a cylinder, you can use the `Cylinder::new` function:

```rust
let cylinder = Cylinder::new(center, radius, height, material);
```

- `center`: A `Vec3` representing the center of the base of the cylinder.
- `radius`: A `f64` representing the radius of the cylinder.
- `height`: A `f64` representing the height of the cylinder.
- `material`: A `Material` struct representing the color of the cylinder.

## How to Adjust Brightness

The brightness of the objects is determined by the `color` field of the `Material` struct. The `color` is a `Vec3` where each component (x, y, z) corresponds to the R, G, and B values of the color, respectively. The values range from 0.0 to 1.0.

To make an object dimmer, you can reduce the values of the `color` vector. For example, to make a red object dimmer, you can change `Vec3::new(1.0, 0.0, 0.0)` to `Vec3::new(0.5, 0.0, 0.0)`.

## How to Move the Camera

The camera is set up in each `scene` function. To move the camera, you need to modify the arguments passed to the `Camera::new` function:

```rust
let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio);
```

- `lookfrom`: A `Vec3` representing the position of the camera.
- `lookat`: A `Vec3` representing the point the camera is looking at.
- `vup`: A `Vec3` representing the "up" direction for the camera.
- `vfov`: A `f64` representing the vertical field of view in degrees.
- `aspect_ratio`: A `f64` representing the aspect ratio of the image.
