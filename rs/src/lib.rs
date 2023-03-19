mod renderer;

use wasm_bindgen::prelude::*;

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
    renderer: renderer::Renderer,
}

#[wasm_bindgen]
impl App {
    pub fn new(
        window: web_sys::Window, 
        canvas: web_sys::HtmlCanvasElement
    ) -> App {
        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

        let renderer = renderer::Renderer::new(&canvas).unwrap();

        App{window, canvas, renderer}
    }

    pub fn render(&self) {
        self.renderer.draw();
    }
}
