pub mod camera;
pub mod cube;
pub mod cylinder;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod plane;
pub mod ray;
pub mod sphere;
pub mod vec3;

use camera::Camera;
use cube::Cube;
use cylinder::Cylinder;
use hittable::Hittable;
use hittable_list::HittableList;
use material::Material;
use plane::Plane;
use ray::Ray;
use sphere::Sphere;
use std::env;
use vec3::Vec3;

const ASPECT_RATIO: f64 = 4.0 / 3.0;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let light_pos = Vec3::new(10.0, 14.0, 10.0);
        let to_light = (light_pos - rec.p).unit_vector();
        let shadow_ray = Ray::new(rec.p, to_light);
        if let Some(_shadow_rec) = world.hit(&shadow_ray, 0.001, f64::INFINITY) {
            return rec.material.color * 0.1; // In shadow
        }

        let ambient = 0.1;
        let diffuse = rec.normal.dot(to_light).max(0.0);
        let mut color = rec.material.color * (ambient + diffuse);
        color.x = color.x.min(1.0);
        color.y = color.y.min(1.0);
        color.z = color.z.min(1.0);

        return color;
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn scene1() -> (HittableList, Camera) {
    let material_red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
    };
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_red);
    let mut world = HittableList::new();
    world.add(Box::new(sphere));
    let cam = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        ASPECT_RATIO, //16.0 / 9.0,
    );
    (world, cam)
}

fn scene2() -> (HittableList, Camera) {
    let material_blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
    };
    let material_green = Material {
        color: Vec3::new(0.0, 1.0, 0.0),
    };
    let plane = Plane::new(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        material_green,
    );
    let cube = Cube::new(
        Vec3::new(-0.25, -0.25, -0.75),
        Vec3::new(0.25, 0.25, -0.25),
        material_blue,
    );
    let mut world = HittableList::new();
    world.add(Box::new(plane));
    world.add(Box::new(cube));
    let cam = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        ASPECT_RATIO,
    );
    (world, cam)
}

fn scene3() -> (HittableList, Camera) {
    let material_red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
    };
    let material_green = Material {
        color: Vec3::new(0.0, 1.0, 0.0),
    };
    let material_blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
    };
    let material_yellow = Material {
        color: Vec3::new(1.0, 1.0, 0.0),
    };
    let material_brown = Material {
        color: Vec3::new(0.75, 0.5, 0.5),
    };

    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_red);
    let sphere2 = Sphere::new(Vec3::new(-0.6, -1.0, -0.5), 0.7, material_brown.clone());
    let sphere3 = Sphere::new(Vec3::new(-0.6, 0.0, -0.5), 0.1, material_brown.clone());
    let sphere4 = Sphere::new(Vec3::new(-0.6, 0.3, -0.5), 0.1, material_brown.clone());
    let sphere5 = Sphere::new(Vec3::new(-0.6, 0.6, -0.5), 0.1, material_brown.clone());
    let sphere6 = Sphere::new(Vec3::new(-0.6, 0.9, -0.5), 0.1, material_brown.clone());

    let plane = Plane::new(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        material_green,
    );
    let cube = Cube::new(
        Vec3::new(-2.0, -0.3, -2.0),
        Vec3::new(-1.0, 0.7, -1.0),
        material_blue,
    );
    let cylinder = Cylinder::new(Vec3::new(1.0, -0.5, -2.0), 0.5, 1.5, material_yellow);

    let mut world = HittableList::new();
    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere3));
    world.add(Box::new(sphere4));
    world.add(Box::new(sphere5));
    world.add(Box::new(sphere6));
    world.add(Box::new(plane));
    world.add(Box::new(cube));
    world.add(Box::new(cylinder));

    let cam = Camera::new(
        Vec3::new(0.2, 0.0, 0.1),
        Vec3::new(-0.1, 0.15, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        ASPECT_RATIO,
    );
    (world, cam)
}

fn scene4() -> (HittableList, Camera) {
    let (world, _) = scene3();
    let cam = Camera::new(
        Vec3::new(-1.0, 0.55, 0.7),
        Vec3::new(-0.65, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        75.0,
        ASPECT_RATIO,
    );
    (world, cam)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene_number = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };

    let (world, cam) = match scene_number {
        1 => scene1(),
        2 => scene2(),
        3 => scene3(),
        4 => scene4(),
        _ => scene1(),
    };

    //let aspect_ratio = 4.0 / 3.0;
    let image_width = 800;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = f64::from(i) / (f64::from(image_width) - 1.0);
            let v = f64::from(j) / (f64::from(image_height) - 1.0);
            let r = cam.get_ray(u, v);
            let pixel_color = ray_color(&r, &world);

            let ir = (255.999 * pixel_color.x) as i32;
            let ig = (255.999 * pixel_color.y) as i32;
            let ib = (255.999 * pixel_color.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
