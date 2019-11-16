// What is stratification? AA p19
//
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use std::ops::{Mul, Div, DivAssign, Add, AddAssign, Sub};
use rand::Rng;

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn calculate_ray(&self, u_right: f64, v_up: f64) -> Ray {
        // borrow origin instead of making a clone! todo
        Ray {
            start: self.origin.clone(),
            direction: &self.lower_left_corner + u_right * &self.horizontal + v_up * &self.vertical 
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin : Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            lower_left_corner : Vec3 { x: -2.0, y: -1.0, z: -1.0 },
            horizontal : Vec3 { x: 4.0, y: 0.0, z: 0.0 },
            vertical : Vec3 { x: 0.0, y: 2.0, z: 0.0 },
        }
    }
}

struct HitableList<'a> {
    hitables: Vec<&'a Hitable>,
}

struct HitRecord<'a> {
    time: f64,
    point: Vec3,
    normal: Vec3,
    material: &'a Material,
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest = t_max;

        for hitable in self.hitables.iter() {
            if let Some(hitrecord) = hitable.hit(ray, t_min, closest) {
                closest = hitrecord.time;
                result = Some(hitrecord);
            }
        }

        result
    }
}

trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere<'a> {
    center: Vec3,
    radius: f64,
    material: &'a Material,
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = &ray.start - &self.center;
        let a: f64 = dot(&ray.direction, &ray.direction);
        let b: f64 = dot(&oc, &ray.direction);
        let c: f64 = dot(&oc, &oc) - self.radius.powi(2);
        let discriminant = b*b - a*c;

        if discriminant > 0.0 {
            let temp: f64 = (- b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.eval(temp);
                return Some(HitRecord {
                    time: temp,
                    // todo wtf...the order matters?
                    normal: (&point - &self.center) / self.radius,
                    point,
                    material: self.material,
                })
            }

            let temp: f64 = (- b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.eval(temp);
                return Some(HitRecord {
                    time: temp,
                    normal: (&p - &self.center) / self.radius,
                    point: p,
                    material: self.material,
                })
            }
        }

        None
    }
}

//// todo i need testing for these fuckers...how do i set it up?
//    // todo the way i set these up is probably a bad idea...they all create new objects
//    // that might be inefficient in many cases
#[derive(Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn normalize(&self) -> Vec3 {
        (1.0/self.length()) * self
    }

    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vec3 {x: self.x * scalar,
               y: self.y * scalar,
               z: self.z * scalar}
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::Output {x: self.x * scalar,
                      y: self.y * scalar,
                      z: self.z * scalar}
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        vec * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: &Vec3) -> Self::Output {
        vec * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0/scalar)
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

// todo repetitive
impl Color {
    fn ri(&self) -> u8 {
        (self.r*255.99) as u8
    }
    fn gi(&self) -> u8 {
        (self.g*255.99) as u8
    }
    fn bi(&self) -> u8 {
        (self.b*255.99) as u8
    }
    fn ai(&self) -> u8 {
        (self.a*255.99) as u8
    }
    fn bytes(&self) -> [u8; 4] {
        [self.ri(),
         self.gi(),
         self.bi(),
         self.ai()]
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Self {
        Color {
            r: scalar * self.r,
            g: scalar * self.g,
            b: scalar * self.b,
            a: scalar * self.a,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Self::Output {
        color * self
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        };
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, scalar: f64) {
        *self = Self {
            r: self.r / scalar,
            g: self.g / scalar,
            b: self.b / scalar,
            a: self.a / scalar,
        };
    }
}

struct Ray {
    start: Vec3,
    direction: Vec3,
}

impl Ray {
    fn eval(&self, t: f64) -> Vec3 {
        &self.start + t*&self.direction
    }

    fn get_color(&self, world: &Hitable, depth: u8) -> Color {
        // todo: setting the minimum to 0.001 is supposed to prevent shadow acne O_o
        // the maximum ought to be something like MAX_FLOAT whatever it's called in rust
        if let Some(hitrecord) = world.hit(&self, 0.001, 99999999.0) {
            if depth < 50 {
                if let Some((attenuation, scattered)) = hitrecord.material.scatter(&self, &hitrecord) {
                    let mut color = attenuation * scattered.get_color(world, depth+1);
                    color.a = 1.0;

                    return color;
                }
            }

            Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
        } else {
            let unit_direction = self.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);

            let mut color = (1.0 - t)*Color{r: 1.0, g: 1.0, b: 1.0, a: 1.0} + t*Color{r: 0.5, g: 0.7, b: 1.0, a: 1.0};
            color.a = 1.0;

            color
        }
    }
}

trait Material {
    // todo can i add names for the parts of the return value?
    fn scatter(&self, ray: &Ray, hitrecord: &HitRecord) -> Option<(Color, Ray)>;
}

struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hitrecord: &HitRecord) -> Option<(Color, Ray)> {
        let target = &hitrecord.point + &hitrecord.normal + random_in_unit_sphere();
        let scattered = Ray {
            direction: target - &hitrecord.point,
            start: hitrecord.point.clone(),
        };

        Some((self.albedo.clone() , scattered)) // todo: clones make me nervous...funny given that all my basic operators create clones x)
    }
}

struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hitrecord: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&ray.direction.normalize(), &hitrecord.normal);
        let scattered = Ray {
            direction: reflected + self.fuzz*random_in_unit_sphere(),
            start: hitrecord.point.clone(),
        };
        if dot(&scattered.direction, &hitrecord.normal) > 0.0 {
            Some((self.albedo.clone() , scattered))
        } else {
            None
        }
    }
}

fn dot(a: &Vec3, b: &Vec3) -> f64 {
    a.x*b.x + a.y*b.y + a.z*b.z
}

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    v - 2.0*dot(v,normal)*normal
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3 {
            x: rand::thread_rng().gen::<f64>(),
            y: rand::thread_rng().gen::<f64>(),
            z: rand::thread_rng().gen::<f64>(),
        } - &Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        if p.length() < 1.0 {
            return p;
        }
    }
}

#[wasm_bindgen]
pub struct Canvas {
    width: u32,
    height: u32,
    buf: Vec<u8>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn buf(&self) -> *const u8 {
        self.buf.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn draw(&mut self) {
        let mut ht: Vec<&Hitable> = Vec::new(); // works
        ht.push(&Sphere {
            center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: &Lambertian { albedo: Color { r: 0.8, g: 0.3, b: 0.3, a: 1.0 } },
        });
        ht.push(&Sphere {
            center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
            radius: 100.0,
            material: &Lambertian { albedo: Color { r: 0.8, g: 0.8, b: 0.0, a: 1.0 } },
        });
        ht.push(&Sphere {
            center: Vec3 { x: 1.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: &Metal { albedo: Color { r: 0.8, g: 0.6, b: 0.2, a: 1.0 }, fuzz: 1.0 },
        });
        ht.push(&Sphere {
            center: Vec3 { x: -1.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: &Metal { albedo: Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 }, fuzz: 0.3 },
        });
        let world = HitableList { 
            hitables: ht,
        };

        let camera = Camera::default();
        let ns = 100;
        for row in 0..self.height {
            for col in 0..self.width {
                let i = (row * self.width + col) as usize;
                // some sampling for antialiasing
                let mut color_sum = Color{ r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
                for _s in 0..ns {
                    let u_offset = rand::thread_rng().gen::<f64>();
                    let v_offset = rand::thread_rng().gen::<f64>();
                    let u_right: f64 = (col as f64 + u_offset) / (self.width as f64);
                    let v_up: f64 = ((self.height - row) as f64 + v_offset) / (self.height as f64);
                    let ray = camera.calculate_ray(u_right, v_up);

                    let color = ray.get_color(&world, 0);
                    color_sum += color;

                }
                color_sum /= ns as f64;
                color_sum.r = color_sum.r.sqrt();
                color_sum.g = color_sum.g.sqrt();
                color_sum.b = color_sum.b.sqrt();
                color_sum.a = 1.0;
                self.buf[4 * i .. 4 * i + 4].copy_from_slice(&color_sum.bytes());
            }
        }
    }

    pub fn new() -> Canvas {
        console_error_panic_hook::set_once();
        let width = 200u32;
        let height = 100u32;
        let buf = vec![0; (width * height) as usize * 4];
        Canvas { width, height, buf }
    }
}
