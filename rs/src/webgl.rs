use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram, WebGlBuffer};
use std::collections::HashMap;

pub struct Renderer {
    pub gl: WebGl2RenderingContext,
}

impl Renderer {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;
        
       Ok(Renderer{gl})
    }

    pub fn draw(
        &self,
        material: &Material,
        geometry: &Geometry,
    ) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        
        self.gl.use_program(Some(&material.program));

        for (name, vertex_buffer) in &geometry.vertex_buffers {
            match material.attributes.get(name) {
                Some(&ref attribute) => {
                    self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
                    self.gl.enable_vertex_attrib_array(attribute.location);
                    self.gl.vertex_attrib_pointer_with_i32(
                        attribute.location, 
                        attribute.size, 
                        WebGl2RenderingContext::FLOAT,
                        false,
                        0,
                        0);
                },
                _ => {}
            }
        }
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&geometry.index_buffer));

        self.gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, geometry.index_length, WebGl2RenderingContext::UNSIGNED_BYTE, 0);

        self.gl.flush();
    }
}

struct Attribute {
    location: u32,
    size: i32,
}

pub struct Material {
    program: WebGlProgram,
    attributes: HashMap<String, Attribute>,
}

impl Material {
    pub fn new(
        gl: &WebGl2RenderingContext,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
        attribute_sizes: &HashMap<String, u8>,
    ) -> Result<Self, String> {
        let vertex_shader = Self::compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_shader_source)?;
        let fragment_shader = Self::compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, fragment_shader_source)?;
        let program = Self::link_program(gl, &vertex_shader, &fragment_shader)?;

        let mut attributes = HashMap::new();
        for (name, size) in attribute_sizes {
            let location = gl.get_attrib_location(&program, &name) as u32;
            attributes.insert(name.clone(), Attribute{location, size: *size as i32});
        }

        Ok(Self{program, attributes})
    }

    fn compile_shader(
        gl: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);
        
        if gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        gl: &WebGl2RenderingContext,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        gl.attach_shader(&program, vertex_shader);
        gl.attach_shader(&program, fragment_shader);
        gl.link_program(&program);

        if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}

pub struct Geometry {
    vertex_buffers: HashMap<String, WebGlBuffer>,
    index_buffer: WebGlBuffer,
    index_length: i32,
}

impl Geometry {
    pub fn new(
        gl: &WebGl2RenderingContext,
        vertex_data: &HashMap<String, &[f32]>,
        index_data: &[u8],
    ) -> Result<Self, String> {
        let mut vertex_buffers = HashMap::new();
        for (name, data) in vertex_data {
            let vertex_buffer = Self::create_vertex_buffer(gl, data)?;
            vertex_buffers.insert(name.clone(), vertex_buffer);
        }
        let index_buffer = Self::create_index_buffer(gl, index_data)?;
        let index_length = index_data.len() as i32;
        
        Ok(Self{vertex_buffers, index_buffer, index_length})
    }

    fn create_vertex_buffer(
        gl: &WebGl2RenderingContext,
        data: &[f32],
    ) -> Result<WebGlBuffer, String> {
        let vertex_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let array_buffer_view = js_sys::Float32Array::view(data);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Ok(vertex_buffer)
    }

    fn create_index_buffer(
        gl: &WebGl2RenderingContext,
        data: &[u8],
    ) -> Result<WebGlBuffer, String> {
        let index_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        unsafe {
            let array_buffer_view = js_sys::Uint8Array::view(data);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);

        Ok(index_buffer)
    }
}

