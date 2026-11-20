use crate::geometry::{
    Point3, Ray,
    color::Color3,
    vec3::{MulS, Vec3},
};

#[derive(Debug)]
enum NetpbmVar {
    PPM,
    _PBM,
    _PGM,
}

/// Main PPM image format type, able to be serialized into the official binary format.
#[derive(Debug)]
pub struct PPM {
    _type: NetpbmVar, // Always PPM for this usecase
    size: (usize, usize),
    aspect_ratio: f64, // Aspect ratio is only "ideal", in the sense that it does not certainly reflect the real ratio of the image, as height is constrainted to be an integer >=1... Thus, viewport sizze cannot be computed from using aspect ratio directly.
    px: Vec<Color3>,   // size.0 * size.1 vector size
}

impl PPM {
    pub fn new(width: usize, aspect_ratio: f64) -> Self {
        let height = (width as f64 / aspect_ratio) as usize; // Aspect
        PPM {
            _type: NetpbmVar::PPM,
            size: if height < 1 {
                (1, width)
            } else {
                (height, width)
            },
            aspect_ratio,
            px: Vec::from_fn(width * height, |_| Color3::new(0., 0., 0.)),
        }
    }

    pub fn get_px(&self, c: (usize, usize)) -> &Color3 {
        &self.px[(c.0 * self.size.1 + c.1) as usize]
    }

    pub fn set_px(&mut self, c: (usize, usize), px: Color3) {
        self.px[(c.0 * self.size.1 + c.1) as usize] = px;
    }

    // Alter "self" image in place
    pub fn render(&mut self) {
        let camera = Camera::new(1.0, 2.0, Vec3(0., 0., 0.), self.size);
        for i in 0..self.size.0 {
            eprintln!("Scanlines remaining: {}", self.size.0 - i); // On stderr for now, may redirect to some other stream later on
            for j in 0..self.size.1 {
                let pixel_center = camera.pix00_loc
                    + camera.pixel_delta_u.muls(i as f64)
                    + camera.pixel_delta_v.muls(j as f64);
                let ray_direction = pixel_center - camera.center;
                let ray = Ray::new(camera.center, ray_direction);
                let colour = ray.ray_color();
                self.set_px((i, j), colour);
            }
        }
    }

    pub fn write(&self) {
        let magic = if let NetpbmVar::PPM = self._type {
            3
        } else {
            panic!("Wrong Netpbm format type...");
        };

        let mut contents = String::new();
        for i in 0..self.size.0 {
            eprintln!("Scanlines remaining: {}", self.size.0 - i); // On stderr for now, may redirect to some other stream later on
            for j in 0..self.size.1 {
                contents.push_str(&format!("{}\n", self.get_px((i, j)).write()));
            }
        }

        let contents = format!(
            "P{}\n{} {}\n{}\n{}",
            magic, self.size.1, self.size.0, 255, &contents
        );
        println!("{}", contents);
    }
}

pub fn rainbow_test(width: usize, aspect_ratio: f64) {
    let mut img = PPM::new(width, aspect_ratio);
    for i in 0..img.size.0 {
        for j in 0..img.size.1 {
            let px = Color3::new(
                i as f64 / (img.size.1 - 1) as f64,
                j as f64 / (img.size.0 - 1) as f64,
                0.,
            );
            img.set_px((i, j), px);
        }
    }

    img.write();
}

#[derive(Debug)]
pub struct Camera {
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    viewport_upper_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,
    pix00_loc: Point3,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        viewport_height: f64,
        center: Point3,
        img: (usize, usize),
    ) -> Self {
        let viewport_width = viewport_height * (img.1 as f64 / img.0 as f64);
        let viewport_u = Vec3(viewport_height, 0., 0.);
        let viewport_v = Vec3(0., -viewport_height, 0.);
        let pixel_delta_u = viewport_u.muls(1. / (img.1 as f64));
        let pixel_delta_v = viewport_v.muls(1. / (img.0 as f64));
        let viewport_upper_left =
            center - Vec3(0., 0., focal_length) - viewport_u.muls(0.5) - viewport_v.muls(0.5);

        Camera {
            focal_length,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            viewport_upper_left,
            pixel_delta_u,
            pixel_delta_v,
            center,
            pix00_loc: viewport_upper_left + (pixel_delta_u + pixel_delta_v).muls(0.5),
        }
    }
}

#[cfg(test)]
mod test {
    // use crate::img::rainbow_test;
}
