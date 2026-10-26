/// Main PPM image format type, able to be serialized into the official binary format.

#[derive(Debug)]
pub struct Pixel(u8, u8, u8);

#[derive(Debug)]
enum NetpbmVar {
    PPM,
    _PBM,
    _PGM,
}

#[derive(Debug)]
pub struct PPM {
    _type: NetpbmVar, // Always PPM for this usecase
    size: (usize, usize),
    px: Vec<Pixel>, // size.0 * size.1 vector size
}

impl PPM {
    pub fn new(size: (usize, usize)) -> Self {
        PPM {
            _type: NetpbmVar::PPM,
            size,
            px: Vec::from_fn(size.0 * size.1, |_| Pixel(0, 0, 0)),
        }
    }

    pub fn get_px(&self, c: (usize, usize)) -> &Pixel {
        &self.px[(c.0 * self.size.1 + c.1) as usize]
    }

    pub fn set_px(&mut self, c: (usize, usize), px: Pixel) {
        self.px[(c.0 * self.size.1 + c.1) as usize] = px;
    }

    // Seems overkill to bring in serde for such a simple serialization model...
    pub fn serialize(&self) -> Result<(), std::io::Error> {
        let magic = if let NetpbmVar::PPM = self._type {
            3
        } else {
            panic!("Wrong Netpbm format type...");
        };

        let mut contents = String::new();
        for i in 0..self.size.0 {
            eprintln!("Scanlines remaining: {}", self.size.0 - i); // On stderr for now, may redirect to some other stream later on
            for j in 0..self.size.1 {
                let Pixel(r, g, b) = self.get_px((i, j));
                contents.push_str(&format!("{} {} {}\n", r, g, b));
            }
        }

        let contents = format!(
            "P{}\n{} {}\n{}\n{}",
            magic, self.size.0, self.size.1, 255, &contents
        );
        println!("{}", contents);
        Ok(())
    }
}

pub fn rainbow_test(size: (usize, usize)) {
    let mut img = PPM::new(size);
    for i in 0..img.size.0 {
        for j in 0..img.size.1 {
            let px = Pixel(
                (255.999 * i as f64 / (img.size.1 - 1) as f64) as u8,
                (255.999 * j as f64 / (img.size.0 - 1) as f64) as u8,
                0,
            );
            img.set_px((i, j), px);
        }
    }

    img.serialize().unwrap();
}

#[cfg(test)]
mod test {
    use crate::img::rainbow_test;

    #[test]
    fn blank_ppm_test() {}

    #[test]
    fn rb_test_1() {
        rainbow_test((255, 255));
    }
}
