use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn apply_filter(width: u32, height: u32) -> Result<(), JsValue> {
    let context = get_canvas_context("canvas");
    let data = get_canvas_context_image_data(&context, width, height);
    let data = get_filtered_image(data, width, height);
    context.put_image_data(&data, 0.0, 0.0)
}

fn get_canvas_context(canvas_id: &str) -> CanvasRenderingContext2d {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id(canvas_id).unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();

  return canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();
}

fn get_canvas_context_image_data(context: &CanvasRenderingContext2d, width: u32, height: u32) -> ImageData {
  return context.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
}

fn get_filtered_image(image_data: ImageData, width: u32, height: u32) -> ImageData {
  let mut data = Vec::new();
  let pixels = image_data.data();

  for y in 0..width {
    for x in 0..height {
      let rgba = get_rgba_in_pos(&pixels, x, y, width);

      push_pixel(&mut data, 255 - rgba.0, 255 - rgba.1, 255 - rgba.2);
    }
  }

  ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height).unwrap()
}

fn push_pixel(data: &mut Vec<u8>, r: u8, g: u8, b: u8) {
  data.push(r);
  data.push(g);
  data.push(b);
  data.push(255); // alpha
}

fn get_rgba_in_pos(image_data_vec: &Clamped<Vec<u8>>, x: u32, y: u32, width: u32) -> (u8, u8, u8, u8) {
  let red_index: usize = (y * (width * 4) + x * 4) as usize;

  return (
    image_data_vec[red_index],
    image_data_vec[red_index + 1],
    image_data_vec[red_index + 2],
    image_data_vec[red_index + 3]
  );
}