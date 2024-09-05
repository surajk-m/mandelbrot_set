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

pub fn generate_mandelbrot(
    width: u32,
    height: u32,
    max_iter: u32,
    center_x: f64,
    center_y: f64,
    zoom: f64,
) -> Vec<u8> {
    let scale_x = 3.0 / (width as f64 * zoom);
    let scale_y = 2.0 / (height as f64 * zoom);

    // Parallelize the processing of each row
    let pixels: Vec<u8> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width)
                .flat_map(move |x| {
                    let cx = center_x + (x as f64 - width as f64 / 2.0) * scale_x;
                    let cy = center_y + (y as f64 - height as f64 / 2.0) * scale_y;

                    let c = Complex::new(cx, cy);
                    let m = mandelbrot(c, max_iter);

                    let pixel = if m < max_iter {
                        let ratio = m as f64 / max_iter as f64;
                        let red = (255.0 * (ratio).powf(0.5)) as u8;
                        let green = (255.0 * (1.0 - ratio).powf(2.0)) as u8;
                        let blue = (255.0 * (ratio * (1.0 - ratio)).powf(0.5)) as u8;
                        vec![red, green, blue]
                    } else {
                        // Black for points in the Mandelbrot set
                        vec![0, 0, 0]
                    };
                    pixel
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    pixels
}
