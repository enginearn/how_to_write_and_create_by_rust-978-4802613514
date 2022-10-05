use image;

fn main() {
    // let mut img = image::ImageBuffer::new(256u8, 256u8);
    // for (x, y, pixel) in img.enumerate_pixels_mut() {
    //     let v = if (x / 32 + y / 32) % 2 == 0 { 255 } else { 0 };
    //     *pixel = image::Rgb([v, v, v]);
    // }
    // img.save::<Q>("checkered.png").unwrap();

    let white = image::Rgb::<u8>([255, 255, 255]);
    let red = image::Rgb::<u8>([255, 90, 90]);

    let w = 64;

    let draw = |x, y| {
        let (xi, yi) = (x / w, y / w);
        match (xi % 2, yi % 2) {
            (0, 0) | (1, 1) => white,
            (0, 1) | (1, 0) => red,
            (_, _) => panic!("error!"),
        }
    };

    let img = image::ImageBuffer::from_fn(256, 256, draw);
    img.save("checkered.png").unwrap();

}
