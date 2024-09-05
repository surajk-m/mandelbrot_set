use num_complex::Complex;
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

pub fn generate_color(ratio: f64) -> (u8, u8, u8) {
    let red = (255.0 * (ratio).powf(0.5)) as u8;
    let green = (255.0 * (1.0 - ratio).powf(2.0)) as u8;
    let blue = (255.0 * (ratio * (1.0 - ratio)).powf(0.5)) as u8;
    (red, green, blue)
}

pub struct MandelbrotConfig {
    pub width: u32,
    pub height: u32,
    pub max_iter: u32,
    pub center_x: f64,
    pub center_y: f64,
    pub zoom: f64,
}

pub fn generate_mandelbrot(config: &MandelbrotConfig) -> Vec<u8> {
    let scale_x = 3.0 / (config.width as f64 * config.zoom);
    let scale_y = 2.0 / (config.height as f64 * config.zoom);

    // Collect the pixels in parallel
    let pixels: Vec<u8> = (0..config.height)
        .into_par_iter()
        .flat_map(|y| {
            // For RGB values
            let mut row_pixels = Vec::with_capacity((config.width * 3) as usize);
            for x in 0..config.width {
                let cx = config.center_x + (x as f64 - config.width as f64 / 2.0) * scale_x;
                let cy = config.center_y + (y as f64 - config.height as f64 / 2.0) * scale_y;

                let c = Complex::new(cx, cy);
                let m = mandelbrot(c, config.max_iter);

                if m < config.max_iter {
                    let ratio = m as f64 / config.max_iter as f64;
                    let (red, green, blue) = generate_color(ratio);
                    row_pixels.push(red);
                    row_pixels.push(green);
                    row_pixels.push(blue);
                } else {
                    // Black for points in the Mandelbrot set
                    row_pixels.push(0);
                    row_pixels.push(0);
                    row_pixels.push(0);
                }
            }
            row_pixels
        })
        .collect();

    pixels
}
