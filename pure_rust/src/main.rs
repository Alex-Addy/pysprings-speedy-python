use image::{ImageBuffer, Luma};
use clap::{Arg, App};

mod random_blending;

fn main() {
    let matches = App::new("random_blending")
        .arg(Arg::with_name("size").required(true).help("size of image in pixels"))
        .arg(Arg::with_name("iterations").required(true).help("number of iterations to run"))
        .get_matches();
    let size = matches.value_of("size").unwrap().parse().unwrap();
    let iterations = matches.value_of("iterations").unwrap().parse().unwrap();

    let mut img = ImageBuffer::new(size as u32, size as u32);
    let grayscale = match random_blending::random_blending(size, iterations) {
        Ok(grayscale) => grayscale,
        Err(err) => {
            println!("Error while blending: {}", err);
            return;
        }
    };
    println!("Converting raw to image");
    for (x, row) in grayscale.into_iter().enumerate() {
        for (y, c) in row.into_iter().enumerate() {
            let pixel = Luma([c]);
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    println!("Done converting");

    img.save("./output.png").unwrap()
}
