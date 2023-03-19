mod webgl;

use wasm_bindgen::prelude::*;
use webgl::{Renderer, Material, Geometry};
use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct App {
    window: web_sys::Window,
    canvas: web_sys::HtmlCanvasElement,
    renderer: Renderer,
    material: Material,
    geometry: Geometry,
}

#[wasm_bindgen]
impl App {
    pub fn new(
        window: web_sys::Window, 
        canvas: web_sys::HtmlCanvasElement
    ) -> App {
        canvas.set_width(800);
        canvas.set_height(600);

        let renderer = Renderer::new(&canvas).unwrap();

        let mut attributes = HashMap::new();
        attributes.insert(String::from("position"), 2);
        attributes.insert(String::from("color"), 3);
        let material = Material::new(&renderer.gl,
                                    include_str!("./shaders/shader.vert"),
                                    include_str!("./shaders/shader.frag"),
                                    &attributes
                                ).unwrap();

        let mut vertex_data = HashMap::new();
        let positions: &[f32] = &[0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let colors: &[f32] = &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        vertex_data.insert(String::from("position"), positions);
        vertex_data.insert(String::from("color"), colors);
        let index_data: &[u8] = &[0, 1, 2];
        let geometry = Geometry::new(&renderer.gl, &vertex_data, index_data).unwrap();

        log!("as");
        App{window, canvas, renderer, material, geometry}
    }

    pub fn render(&self) {
        self.renderer.draw(&self.material, &self.geometry);
    }
}
