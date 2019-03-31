use image::{ImageBuffer, Luma};

mod random_blending;

const SIZE: usize = 128;

fn main() {
    let mut img = ImageBuffer::new(SIZE as u32, SIZE as u32);
    let canvas = random_blending::random_blending(SIZE, 50);
    println!("Converting raw to image");
    for (x, row) in canvas.into_iter().enumerate() {
        for (y, c) in row.into_iter().enumerate() {
            let pixel = Luma([c]);
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    println!("Done converting");

    img.save("./output.png").unwrap()
}
