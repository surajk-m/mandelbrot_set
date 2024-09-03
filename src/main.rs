use num_complex::Complex;
use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;

fn mandelbrot(c: Complex<f64>, max_iter: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

fn generate_mandelbrot(width: u32, height: u32, max_iter: u32) -> Vec<u8> {
    let scale_x = 3.0 / width as f64;
    let scale_y = 3.0 / height as f64;

    // Parallelize the processing of each row
    let pixels: Vec<u8> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width).flat_map(move |x| {
                let cx = x as f64 * scale_x - 2.0;
                let cy = y as f64 * scale_y - 1.5;

                let c = Complex::new(cx, cy);
                let m = mandelbrot(c, max_iter);

                let pixel = (255 * m / max_iter) as u8;
                vec![pixel, pixel, pixel]
            }).collect::<Vec<u8>>()
        })
        .collect();

    pixels
}

fn render_mandelbrot(width: u32, height: u32, max_iter: u32) {
    let pixels = generate_mandelbrot(width, height, max_iter);
    let img: RgbImage = ImageBuffer::from_raw(width, height, pixels).unwrap();
    img.save("mandelbrot.png").unwrap();
}

fn main() {
    let width = 800;
    let height = 600;
    let max_iter = 1500;
    // let width = 3840;     // 4K resolution width
    // let height = 2160;    // 4K resolution height
    // let max_iter = 5000;  // iterations for more detail

    // let width = 7680;     // 8K resolution width
    // let height = 4320;    // 8K resolution height
    // let max_iter = 10000; // Even more iterations for ultra-fine details

    render_mandelbrot(width, height, max_iter);
    println!("Mandelbrot set generated and saved as 'mandelbrot.png'.");
}

