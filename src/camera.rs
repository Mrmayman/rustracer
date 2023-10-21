use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::pixelbuffer;
use crate::profiler::Profiler;
use crate::random::{self, random_double};
use crate::ray::{degrees_to_radians, Ray};
use crate::vector::{cross, random_in_unit_disk, random_unit_vector, Vec3};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

const DEPTH_OF_FIELD: bool = false;

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

#[derive(Clone)]
pub struct Camera {
    pub pixel_buffer: Box<[u8]>,
    pub pixel_buffer_2: Box<[u8]>,
    aspect_ratio: f64,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    upper_left_pixel_location: Vec3,
    samples_per_pixel: i32,
    max_depth: i32,
    // Extra
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub vfov: f64,
    defocus_angle: f64,
    focus_distance: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let pixel_buffer: Box<[u8]> = pixelbuffer::new();
        let pixel_buffer_2: Box<[u8]> = pixelbuffer::new();

        let aspect_ratio: f64 = (pixelbuffer::WIDTH as f64) / (pixelbuffer::HEIGHT as f64);

        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let focal_length: f64 = 1.0;
        let camera_center: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u: Vec3 = Vec3::new(viewport_width.clone(), 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -(viewport_height.clone()), 0.0);

        let pixel_delta_u: Vec3 = viewport_u / (pixelbuffer::WIDTH as f64);
        let pixel_delta_v: Vec3 = viewport_v / (pixelbuffer::HEIGHT as f64);

        let viewport_upper_left: Vec3 = camera_center
            - Vec3::new(0.0, 0.0, focal_length.clone())
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let upper_left_pixel_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            pixel_buffer: pixel_buffer,
            pixel_buffer_2: pixel_buffer_2,
            aspect_ratio: aspect_ratio,
            camera_center: camera_center,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            upper_left_pixel_location: upper_left_pixel_location,
            samples_per_pixel: 5,
            max_depth: 4,
            lookat: Vec3::new(0.0, 0.0, -1.0),
            lookfrom: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            vfov: 90.0,
            defocus_angle: 10.0,
            focus_distance: 3.4,
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn initialize(&mut self) {
        self.camera_center = self.lookfrom;

        let focal_length: f64 = (self.lookfrom - self.lookat).length();
        let theta: f64 = degrees_to_radians(self.vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64;
        if DEPTH_OF_FIELD {
            viewport_height = 2.0 * h * self.focus_distance;
        } else {
            viewport_height = 2.0 * h * focal_length;
        }
        let viewport_width: f64 =
            viewport_height * (pixelbuffer::WIDTH as f64 / pixelbuffer::HEIGHT as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = cross(self.vup, self.w).unit_vector();
        self.v = cross(self.w, self.u);

        let viewport_u: Vec3 = viewport_width * self.u;
        let viewport_v: Vec3 = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / pixelbuffer::WIDTH as f64;
        self.pixel_delta_v = viewport_v / pixelbuffer::HEIGHT as f64;

        let viewport_upper_left: Vec3;
        if DEPTH_OF_FIELD {
            viewport_upper_left = self.camera_center
                - self.camera_center
                - (self.focus_distance * self.w)
                - viewport_u / 2.0
                - viewport_v / 2.0;
        } else {
            viewport_upper_left =
                self.camera_center - (focal_length * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        }
        self.upper_left_pixel_location =
            viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius =
            self.focus_distance * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&mut self, world: &HittableList, rng: &mut XorShiftRng) {
        self.initialize();
        for y in 0..pixelbuffer::HEIGHT {
            for x in 0..pixelbuffer::WIDTH {
                // println!("Started pixel");
                let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(x as i32, y as i32, rng);
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world, rng);
                }
                self.write_color(pixel_color, self.samples_per_pixel, x, y);
                // println!("Finished pixel");
            }
        }
    }

    fn write_color(&mut self, color: Vec3, samples_per_pixel: i32, pixel_x: usize, pixel_y: usize) {
        let mut r = color.x();
        let mut g = color.y();
        let mut b = color.z();

        let scale = 1.0 / (samples_per_pixel as f64);
        r *= scale;
        g *= scale;
        b *= scale;

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        let new_color = Vec3::new(INTENSITY.clamp(r), INTENSITY.clamp(g), INTENSITY.clamp(b));

        const INTENSITY: Interval = Interval {
            min: 0.0,
            max: 0.999,
        };
        pixelbuffer::set_color(&mut self.pixel_buffer, pixel_x, pixel_y, &new_color);
    }

    fn ray_color(
        &self,
        current_ray: &Ray,
        depth: i32,
        world: &HittableList,
        rng: &mut XorShiftRng,
    ) -> Vec3 {
        let mut temp_hit_record: HitRecord = HitRecord::new();

        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let world_hit: bool = world.hit(
            current_ray.clone(),
            Interval {
                min: 0.001,
                max: std::f64::INFINITY,
            },
            &mut temp_hit_record,
        );

        if !world_hit {
            let unit_direction = current_ray.direction().unit_vector();

            // Converting the ray's y position to between 0.0 and 1.0 in screen space
            let lerped_value = 0.5 * (unit_direction.y() + 1.0);

            const SKY_TOP_COLOR: Vec3 = Vec3 { e: [0.0, 0.0, 0.0] };
            const SKY_BOTTOM_COLOR: Vec3 = Vec3 { e: [0.0, 0.1, 0.3] };

            // Linear interpolation
            return (1.0 - lerped_value) * SKY_BOTTOM_COLOR + lerped_value * SKY_TOP_COLOR;
        }

        let mut scattered: Ray = Ray::new(Vec3::new_default(), Vec3::new_default(), 0.0);
        let mut attenuation: Vec3 = Vec3::new_default();
        let color_from_emission: Vec3 = temp_hit_record.material.emitted(
            temp_hit_record.u,
            temp_hit_record.v,
            &temp_hit_record.point
        );

        if !temp_hit_record.material.scatter(
            current_ray,
            &temp_hit_record,
            &mut attenuation,
            &mut scattered,
            rng,
        ) {
            return color_from_emission;
        }

        let color_from_scatter: Vec3 =
            attenuation * self.ray_color(&scattered, depth - 1, world, rng);

        return color_from_emission + color_from_scatter;
    }

    fn get_ray(&self, x: i32, y: i32, rng: &mut XorShiftRng) -> Ray {
        let pixel_center: Vec3 = self.upper_left_pixel_location
            + (x as f64 * self.pixel_delta_u)
            + (y as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square(rng);

        let ray_origin: Vec3;
        if DEPTH_OF_FIELD && self.defocus_angle > 0.0 {
            ray_origin = self.defocus_disk_sample(rng);
        } else {
            ray_origin = self.camera_center;
        }
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        let ray_time: f64 = random_double(rng);

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self, rng: &mut XorShiftRng) -> Vec3 {
        let p: Vec3 = random_in_unit_disk(rng);
        self.camera_center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self, rng: &mut XorShiftRng) -> Vec3 {
        let px = random::random_double(rng) - 0.5;
        let py = random::random_double(rng) - 0.5;
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}
