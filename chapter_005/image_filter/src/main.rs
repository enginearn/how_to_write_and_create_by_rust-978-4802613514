use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <image>", args[0]);
        return;
    }

    let infile = args[1].clone();
    let outfile = format!("{}-filtered.png", infile);
    println!("Reading {}", infile);
    println!("Writing {}", outfile);

    let mut img = image::open(infile).expect("Failed to open image");
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel: Rgba<u8> = img.get_pixel(x, y);
            let c = Rgba([
                255 - pixel[0], // red
                255 - pixel[1], // green
                255 - pixel[2], // blue
                pixel[3],       // alpha
            ]);
            img.put_pixel(x, y, c);
        }
    }
    img.save(outfile).expect("Failed to save image");
}
