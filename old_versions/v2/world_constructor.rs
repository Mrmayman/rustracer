use std::sync::Arc;

use rand_xorshift::XorShiftRng;

use crate::{
    hittable::{
        bvh_node::BvhNode,
        quad::Quad,
        transformations::{rotation_y::RotateY, translation::Translate},
    },
    material::{lambertian::Lambertian, metal::Metal},
    texture::image_texture::ImageTexture,
    vector::Vec3,
    HittableList,
};

const CAR_COLOR: Vec3 = Vec3 { e: [1.0, 0.2, 0.1] };

pub fn init_world(current_path: String, rng: &mut XorShiftRng) -> HittableList {
    // Add stuff to world
    let mut world: HittableList = HittableList::new();

    construct_world(current_path, &mut world);

    world = HittableList::new_add(Box::new(BvhNode::new_list(&world, rng)));

    construct_car(&mut world);

    world
}

fn construct_world(current_path: String, world: &mut HittableList) {
    let road1 = Box::new(Quad::new(
        Vec3::new(-5.0, 0.0, -5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        Box::new(Lambertian::new_texture(Arc::new(
            ImageTexture::new(&(current_path + "road.png")).expect("Could not load image"),
        ))),
    ));
    world.add(Box::new(RotateY::new(road1, 0.0)));
}

fn add_car_color_quad(world: &mut HittableList, base: Vec3, arm1: Vec3, arm2: Vec3) {
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                base,
                arm1,
                arm2,
                Box::new(Metal::new(CAR_COLOR, 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 0.0),
    )));
}

fn construct_car(world: &mut HittableList) {
    // Car top
    add_car_color_quad(
        world,
        Vec3::new(0.8, 1.0, -1.0),
        Vec3::new(-1.6, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 2.0),
    );

    // Car back glass
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(0.8, 0.6, -2.0),
                Vec3::new(-1.6, 0.0, 0.0),
                Vec3::new(0.0, 0.4, 1.0),
                Box::new(Metal::new(Vec3::new(0.0, 0.0, 0.0), 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 0.0),
    )));

    // Car back top
    add_car_color_quad(
        world,
        Vec3::new(1.0, 0.6, -2.5),
        Vec3::new(-2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.1, 1.5),
    );

    // Car back back
    add_car_color_quad(
        world,
        Vec3::new(1.0, 0.05, -2.7),
        Vec3::new(-2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.55, 0.2),
    );

    // Car front top
    add_car_color_quad(
        world,
        Vec3::new(1.0, 0.6, 1.0),
        Vec3::new(-2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.5),
    );

    // Car right window
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(-1.0, 0.6, -1.0),
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(0.2, 0.4, 0.0),
                Box::new(Metal::new(Vec3::new(0.0, 0.0, 0.0), 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 0.0),
    )));

    // Car right back pad
    add_car_color_quad(
        world,
        Vec3::new(-0.8, 0.7, -2.0),
        Vec3::new(-0.2, -0.3, 0.0),
        Vec3::new(0.0, 0.3, 1.0),
    );

    // Car right door
    add_car_color_quad(
        world,
        Vec3::new(-1.0, 0.05, -2.5),
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.6, 0.0),
    );

    // Car left window
    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(Quad::new(
                Vec3::new(1.0, 0.6, -1.0),
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(-0.2, 0.4, 0.0),
                Box::new(Metal::new(Vec3::new(0.0, 0.0, 0.0), 0.2)),
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 0.0),
    )));

    // Car left back pad
    add_car_color_quad(
        world,
        Vec3::new(0.8, 0.7, -2.0),
        Vec3::new(0.0, 0.3, 1.0),
        Vec3::new(0.2, -0.3, 0.0),
    );

    // Car left door
    add_car_color_quad(
        world,
        Vec3::new(1.0, 0.6, -2.5),
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, -0.55, 0.0),
    );
}
