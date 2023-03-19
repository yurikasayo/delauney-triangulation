use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram};

pub struct Renderer {
    context: WebGl2RenderingContext,
    width: f64,
    height: f64,
}

impl Renderer {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        let vertex_shader = Self::compile_shader(
            &context, 
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("./shaders/shader.vert")
        )?;
        let fragment_shader = Self::compile_shader(
            &context, 
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("./shaders/shader.frag")
        )?;
        let program = Self::link_program(&context, &vertex_shader, &fragment_shader)?;
        context.use_program(Some(&program));

        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

        let position_attribute_location = context.get_attrib_location(&program, "position");
        let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);
        context.bind_vertex_array(Some(&vao));
        
       Ok(Renderer{context, width, height})
    }

    pub fn draw(&self) {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
    }

    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);
        
        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        context: &WebGl2RenderingContext,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vertex_shader);
        context.attach_shader(&program, fragment_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}