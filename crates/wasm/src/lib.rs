use ray_tracer_core::{draw::Canvas, scenes};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RenderResult {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl RenderResult {
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[wasm_bindgen(getter)]
    pub fn pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }
}

#[wasm_bindgen]
pub fn render_projectile() -> RenderResult {
    let canvas = scenes::projectile::render_projectile();
    to_render_result(canvas)
}

#[wasm_bindgen]
pub fn render_clock() -> RenderResult {
    let canvas = scenes::render_clock();
    to_render_result(canvas)
}

#[wasm_bindgen]
pub fn render_circle() -> RenderResult {
    let canvas = scenes::render_circle();
    to_render_result(canvas)
}

fn to_render_result(canvas: Canvas) -> RenderResult {
    let pixels = to_rgba(&canvas);

    RenderResult {
        width: canvas.width,
        height: canvas.height,
        pixels,
    }
}

fn to_rgba(canvas: &Canvas) -> Vec<u8> {
    let mut data = Vec::with_capacity(canvas.width * canvas.height * 4);

    for y in 0..canvas.height {
        for x in 0..canvas.width {
            let color = canvas[(x, y)];
            data.push(to_byte(color.r));
            data.push(to_byte(color.g));
            data.push(to_byte(color.b));
            data.push(255);
        }
    }

    data
}

fn to_byte(value: f64) -> u8 {
    let clamped = value.clamp(0.0, 1.0);
    (clamped * 255.0).round() as u8
}
