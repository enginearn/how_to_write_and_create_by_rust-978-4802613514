use image::{self, imageops, GenericImageView};

fn main() {
    let size = 128;
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[Usage]: {} <image>", args[0]);
        return;
    }

    let infile = String::from(&args[1]);
    let outfile = format!("thumb_{}.png", infile);
    println!("{} -> {}", infile, outfile);

    let mut img1 = image::open(infile).expect("Failed to open image");
    let dim = img1.dimensions();

    let w = if dim.0 > dim.1 { dim.1 } else { dim.0 };
    let mut img2 = imageops::crop(&mut img1, (dim.0 - w) / 2, (dim.1 - w) / 2, w, w).to_image();
    let img3 = imageops::resize(&mut img2, size, size, imageops::FilterType::Lanczos3);

    img3.save(outfile).expect("Failed to save image");
}
