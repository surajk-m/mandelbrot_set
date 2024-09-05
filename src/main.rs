use image::{ImageBuffer, RgbImage};

mod mandelbrot;
use mandelbrot::generate_mandelbrot;

fn render_mandelbrot(
    width: u32,
    height: u32,
    max_iter: u32,
    center_x: f64,
    center_y: f64,
    zoom: f64,
) {
    let pixels = generate_mandelbrot(width, height, max_iter, center_x, center_y, zoom);
    let img: RgbImage = ImageBuffer::from_raw(width, height, pixels).unwrap();
    img.save("mandelbrot.png").unwrap();
}

fn main() {
    // let width = 800;
    // let height = 600;
    // let max_iter = 1500;

    let width = 3840; // 4K resolution width
    let height = 2160; // 4K resolution height
    let max_iter = 5000; // more iterations for ultra-fine details

    // let width = 7680;       // 8K resolution width
    // let height = 4320;      // 8K resolution height
    // let max_iter = 10000;

    // let width = 15360;      // 16K resolution width
    // let height = 8640;      // 16K resolution height
    // let max_iter = 10000;

    // Center of the zoom and zoom level
    let center_x = -0.5;
    let center_y = 0.0;
    let zoom = 1.0;

    // let center_x = -0.75;
    // let center_y = 0.1;
    // let zoom = 100.0; // 100x zoom into a specific area of the set

    render_mandelbrot(width, height, max_iter, center_x, center_y, zoom);
    println!("Mandelbrot set generated and saved as 'mandelbrot.png'.");
}
