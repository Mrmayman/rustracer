use crate::material::{Metal, Lambertian};
use crate::hittable::{Translate, RotateY, Quad, HittableList};
use crate::texture::ImageTexture;
use std::rc::Rc;
use crate::vector::Vec3;

pub(crate) fn construct_world(current_path: String, world: &mut HittableList) {
    let road1 = Box::new(Quad::new(
        Vec3::new(-5.0, 0.0, -5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        Rc::new(Lambertian::new_texture(Rc::new(
            ImageTexture::new(&(current_path + "road.png")).expect("Could not load image"),
        ))),
    ));
    world.add(Box::new(RotateY::new(road1, 0.0)));
}

pub(crate) fn construct_car(world: &mut HittableList) {
    let car_color = Vec3::new(1.0, 0.2, 0.1);

    // Car top
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(0.8, 1.0, -1.0),
                Vec3::new(-1.6, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 2.0),
                Rc::new(Metal::new(&car_color, 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car back glass
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(0.8, 0.6, -2.0),
                Vec3::new(-1.6, 0.0, 0.0),
                Vec3::new(0.0, 0.4, 1.0),
                Rc::new(Metal::new(&Vec3::new(0.0, 0.0, 0.0), 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car back top
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(1.0, 0.6, -2.5),
                Vec3::new(-2.0, 0.0, 0.0),
                Vec3::new(0.0, 0.1, 1.5),
                Rc::new(Metal::new(&car_color, 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car back back
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(1.0, 0.05, -2.7),
                Vec3::new(-2.0, 0.0, 0.0),
                Vec3::new(0.0, 0.55, 0.2),
                Rc::new(Metal::new(&car_color, 0.0)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car front top
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(1.0, 0.6, 1.0),
                Vec3::new(-2.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.5),
                Rc::new(Metal::new(&car_color, 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car right window
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(-1.0, 0.6, -1.0),
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(0.2, 0.4, 0.0),
                Rc::new(Metal::new(&Vec3::new(0.0, 0.0, 0.0), 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car right back pad
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(-0.8, 0.7, -2.0),
                Vec3::new(-0.2, -0.3, 0.0),
                Vec3::new(0.0, 0.3, 1.0),
                Rc::new(Metal::new(&car_color, 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car right door
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(-1.0, 0.05, -2.5),
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::new(0.0, 0.6, 0.0),
                Rc::new(Metal::new(&car_color, 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car left window
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(1.0, 0.6, -1.0),
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(-0.2, 0.4, 0.0),
                Rc::new(Metal::new(&Vec3::new(0.0, 0.0, 0.0), 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car left back pad
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(0.8, 0.7, -2.0),
                Vec3::new(0.0, 0.3, 1.0),
                Vec3::new(0.2, -0.3, 0.0),
                Rc::new(Metal::new(&car_color, 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));

    // Car left door
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(1.0, 0.6, -2.5),
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::new(0.0, -0.55, 0.0),
                Rc::new(Metal::new(&car_color, 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 5.0),
    )));
}
