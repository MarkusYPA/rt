# Technical Explanation of the Rust Ray Tracer

This document provides a technical explanation of the logic behind the Rust ray tracer.

## Core Concepts

The program is a basic ray tracer that simulates the path of light to generate an image. The core components are:

- **Rays:** A ray is a half-line with an origin and a direction. In this program, rays are cast from the camera through each pixel of the image plane into the scene.
- **Hittable Objects:** These are the objects in the scene that can be intersected by rays. The program implements four types of hittable objects: spheres, planes, cubes, and cylinders.
- **Camera:** The camera determines the view of the scene. It is responsible for generating the rays that are cast into the scene.
- **Lighting and Shading:** The color of each pixel is determined by the interaction of light with the objects in the scene. The program implements a simple diffuse shading model with shadows.

## Program Structure

The program is structured into several modules:

- **`main.rs`:** This is the main entry point of the program. It is responsible for parsing command-line arguments, setting up the scene, and rendering the image.
- **`vec3.rs`:** This module provides a `Vec3` struct for representing 3D vectors and points, along with common vector operations.
- **`ray.rs`:** This module provides a `Ray` struct for representing rays.
- **`hittable.rs`:** This module defines the `Hittable` trait, which is implemented by all objects that can be intersected by rays.
- **`hittable_list.rs`:** This module provides a `HittableList` struct for storing a list of hittable objects.
- **`sphere.rs`, `plane.rs`, `cube.rs`, `cylinder.rs`:** These modules implement the `Hittable` trait for the four different types of objects.
- **`material.rs`:** This module provides a `Material` struct for representing the color of an object.
- **`camera.rs`:** This module provides a `Camera` struct for generating rays.

## Ray-Object Intersection

The core of the ray tracer is the ray-object intersection logic. For each pixel in the image, a ray is cast from the camera into the scene. The program then checks if this ray intersects with any of the objects in the scene.

If the ray intersects with an object, the program calculates the color of the pixel based on the material of the object and the lighting in the scene. If the ray does not intersect with any objects, the program sets the color of the pixel to the background color.

The intersection logic for each object is implemented in the `hit` method of the `Hittable` trait.

### Sphere

The intersection of a ray and a sphere is calculated by solving a quadratic equation. The equation is derived from the formula for a sphere and the formula for a ray.

### Plane

The intersection of a ray and a plane is calculated by finding the point where the ray and the plane intersect. This is done by solving a linear equation.

### Cube

The intersection of a ray and a cube is calculated using a slab-based intersection test. This method checks for intersections with the six faces of the cube.

### Cylinder

The intersection of a ray and a cylinder is calculated by checking for intersections with the cylindrical body and the top and bottom caps.

## Lighting and Shading

The program implements a simple diffuse shading model. The color of a point on an object is calculated as:

```
color = object_color * (ambient + diffuse)
```

- `object_color`: The color of the object, as defined by its material.
- `ambient`: A constant factor that represents the ambient light in the scene.
- `diffuse`: The diffuse component of the light, which is calculated as the dot product of the surface normal and the direction to the light source.

### Shadows

To create shadows, the program casts a "shadow ray" from the intersection point towards the light source. If this shadow ray intersects with another object before it reaches the light source, the point is considered to be in shadow, and its color is darkened.
