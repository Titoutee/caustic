use caustic::img::{self, PPM};

fn main() {
    let ratio = 16. / 9.;
    let img_width = 256;
    let mut img = PPM::new(img_width, ratio);
    img.render();
    img.write();
}
