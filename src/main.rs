use image::{ImageBuffer, RgbImage};

mod mandelbrot;
use mandelbrot::{generate_mandelbrot, MandelbrotConfig};

fn render_mandelbrot(config: MandelbrotConfig) {
    let pixels = generate_mandelbrot(&config);
    let img: RgbImage = ImageBuffer::from_raw(config.width, config.height, pixels).unwrap();
    img.save("mandelbrot.png").unwrap();
}

fn main() {
    // 4K resolution
    let config = MandelbrotConfig {
        width: 3840,
        height: 2160,
        max_iter: 5000,
        center_x: -0.5,
        center_y: 0.0,
        zoom: 1.0,
    };

    // 8K resolution
    // let config = MandelbrotConfig {
    //     width: 7680,
    //     height: 4320,
    //     max_iter: 10000,
    //     center_x: -0.5,
    //     center_y: 0.0,
    //     zoom: 1.0,
    // };

    // 16K resolution
    // let config = MandelbrotConfig {
    //     width: 15360,
    //     height: 8640,
    //     max_iter: 10000,
    //     center_x: -0.5,
    //     center_y: 0.0,
    //     zoom: 1.0,
    // };

    // let center_x = -0.75;
    // let center_y = 0.1;
    // let zoom = 100.0; // 100x zoom into a specific area of the set

    render_mandelbrot(config);
    println!("Mandelbrot set generated and saved as 'mandelbrot.png'.");
}
