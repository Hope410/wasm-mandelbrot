mod utils;

use wasm_bindgen::prelude::*;
use num::complex::Complex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Range(pub f64, pub f64);

#[wasm_bindgen]
impl Range {
    pub fn new(a: f64, b: f64) -> Range {
        Range (a, b)
    }
}

#[wasm_bindgen]
pub struct Mandelbrot {
    width: u32,
    height: u32,
    iterations: u32,
    threshold: f64,
    colors: Vec<u32>,
}

#[wasm_bindgen]
impl Mandelbrot {
    fn compute_point_value(&self, a: f64, b: f64, power: f64) -> u32 {
        let c: Complex<f64> = Complex::new(a, b);
        let mut z: Complex<f64> = Complex::new(0.0, 0.0);

        for i in 0..self.iterations {
            z = z.powf(power) + c;

            if z.norm() > self.threshold {
                return i;
            }
        }

        return self.iterations - 1;
    }

    pub fn update(&self, range_x: &Range, range_y: &Range, power: f64) -> Vec<u32> {
        let mut data: Vec<u32> = Vec::new();

        let range_x_size: f64 = range_x.1 - range_x.0;
        let scale_x: f64 = (self.width as f64) / range_x_size;
        
        let range_y_size: f64 = range_y.1 - range_y.0;
        let scale_y: f64 = (self.width as f64) / range_y_size;

        for y in 0..self.height {
            for x in 0..self.width {
                let cx: f64 = range_x.0 + (x as f64) / scale_x;
                let cy: f64 = range_y.0 + (y as f64) / scale_y;

                let color_index = self.compute_point_value(cx, cy, power);

                match self.colors.get(color_index as usize) {
                    Some(color) => data.push(*color),
                    None => panic!("Can't get color by index {}", color_index)
                }
            }
        }

        return data;
    }

    pub fn new(width: u32, height: u32, iterations: u32, threshold: f64, colors: Vec<u32>) -> Mandelbrot {
        Mandelbrot {
            width,
            height,
            iterations,
            threshold,
            colors,
        }
    }
}