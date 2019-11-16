use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::f32::consts::PI;
use nalgebra::{Matrix4, Vector3};


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // TODO: call setup?
    Ok(())
}


#[wasm_bindgen]
pub struct GodObject {
    shader_program: Option<web_sys::WebGlProgram>,
    global_context: Option<web_sys::WebGlRenderingContext>,
}


#[wasm_bindgen]
impl GodObject {
    pub fn render(self, timestamp: f32) {
        // Yuck. We don't know the state of this object.
        if self.shader_program.is_none() ||
            self.global_context.is_none() {
                return
            }

        let context = self.global_context.unwrap();
        let program = self.shader_program.unwrap();

        // clear background
        context.clear_color(0.0, 0.0, 0.5, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        // clear depth buffer
        context.clear_depth(1.0);
        context.clear(WebGlRenderingContext::DEPTH_BUFFER_BIT);
        // enable depth testing
        context.enable(WebGlRenderingContext::DEPTH_TEST);
        context.depth_func(WebGlRenderingContext::LEQUAL);

        const FOV: f32 = 0.25 * PI; // field of view
        const ASPECT: f32 = 1.0; // canvas must be square
        const NEAR_Z: f32 = 0.1; // objects closer cannot be seen
        const FAR_Z: f32 = 100.0; // objects further away cannot be seen
        let projmat: Matrix4<f32> = Matrix4::new_perspective(ASPECT, FOV, NEAR_Z, FAR_Z);

        let mut mvmat: Matrix4<f32> = Matrix4::identity();
        let translation = Matrix4::new_translation(&Vector3::new(0.0, 0.0, -6.0));
        mvmat *= translation;
        let mut angle: f32 = timestamp / 1000.0;
        while angle > 2.0*PI { angle -= 2.0*PI };
        let rotation = Matrix4::new_rotation(Vector3::new(0.0, 0.0, 1.0));

        let mvmat_location = context.get_uniform_location(&program, "uModelViewMatrix");
        context.uniform_matrix4fv_with_f32_array(mvmat_location.as_ref(), false, mvmat.as_slice());
        let projmat_location = context.get_uniform_location(&program, "uProjectionMatrix");
        context.uniform_matrix4fv_with_f32_array(projmat_location.as_ref(), false, projmat.as_slice());

        context.draw_arrays(
            WebGlRenderingContext::TRIANGLE_STRIP,
            0,
            4, // TODO this is kind of shitty, we're passing the buffer but we don't know what's in it (vertices.len() / 3) as i32,
        );
    }

    // TODO how do I make this work? this doesn't even get called from js...
    //pub fn setup(self) -> Result<(), ()> {
    pub fn setup(mut self) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("output").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        self.global_context = Some(canvas
            .get_context("webgl").unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>().unwrap());

        // -------------------------------------
        // Create shader program and activate it

        let vert_shader = compile_shader(
            // okay, so unwrapping the Option apparently moves the contents...
            // but we don't really need the option anyway
            &self.global_context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec4 aVertexPosition;
            attribute vec4 aVertexColor;

            uniform mat4 uModelViewMatrix;
            uniform mat4 uProjectionMatrix;

            varying lowp vec4 vColor;

            void main() {
              gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
              vColor = aVertexColor;
            }
            "#,
        ).unwrap();

        let frag_shader = compile_shader(
            &self.global_context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            varying lowp vec4 vColor;

            void main() {
                gl_FragColor = vColor;
            }
        "#,
        ).unwrap();

        let program = link_program(self.global_context, &vert_shader, &frag_shader).unwrap();
        self.global_context.unwrap().use_program(Some(&program));
        self.shader_program = Some(program);

        
        // -------------------------------------
        // create a buffer for the wasm memory
        // somehow this is needed for converting values
        //
        // docs: get a handle to the wasm memory
        let memory_buffer = wasm_bindgen::memory()
            // try to dynamic cast it to the right type?
            .dyn_into::<WebAssembly::Memory>().unwrap()
            // what? it's already a memory pointer...
            .buffer();


        // --------------------------------
        // Set up stuff to color vertices
        const COLORS: [f32; 16] = [
            //1.0,  1.0,  1.0,  1.0,    // white
            1.0,  1.0,  0.0,  1.0,    // yellow
            1.0,  0.0,  0.0,  1.0,    // red
            0.0,  1.0,  0.0,  1.0,    // green
            0.0,  0.0,  1.0,  1.0,    // blue
        ];
        let color_buffer: web_sys::WebGlBuffer = self.global_context.unwrap().create_buffer().ok_or("Failed to create color buffer.").unwrap();
        self.global_context.unwrap().bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));

        let colors_location = COLORS.as_ptr() as u32 / 4;
        let colors_js_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(colors_location, colors_location + COLORS.len() as u32);


        // -------------------------
        // Add vertices for a square

        //                  +y
        //                   ^
        //                   |
        // +----------------------------------+
        // |                 |                |
        // |                 |                |
        // |                 |                |
        // | -  -  -  -  - (0/0)  -  -  -  -  |  --> +x
        // |                 |                |
        // |                 |                |
        // |                 |                |
        // +----------------------------------+

        const LOW: f32 = -1.5;
        const HIGH: f32 = 1.5;
        let vertices: [f32; 12] = [ LOW,  HIGH, 0.0,
                                    LOW,  LOW,  0.0,
                                    HIGH, HIGH, 0.0,
                                    HIGH, LOW,  0.0];
        
        let vertices_location = vertices.as_ptr() as u32 / 4;

        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

        let buffer = self.global_context.unwrap().create_buffer().ok_or("failed to create buffer").unwrap();

        // bind this buffer with vertex coords, makes it "active"
        self.global_context.unwrap().bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        // "initializes the buffer object's data store"
        self.global_context.unwrap().buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        // now we apply the active buffer to our vertices
        let vertex_location = self.global_context.unwrap().get_attrib_location(&program, "aVertexPosition");
        self.global_context.unwrap().vertex_attrib_pointer_with_i32(vertex_location as u32, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.global_context.unwrap().enable_vertex_attrib_array(vertex_location as u32);

        // and now the same again for colors
        self.global_context.unwrap().bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        self.global_context.unwrap().buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &colors_js_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
        let color_location = self.global_context.unwrap().get_attrib_location(&program, "aVertexColor");
        self.global_context.unwrap().vertex_attrib_pointer_with_i32(color_location as u32, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.global_context.unwrap().enable_vertex_attrib_array(color_location as u32);

    }

}

pub fn compile_shader(
    context: &Option<WebGlRenderingContext>,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let context2: &WebGlRenderingContext = context.as_ref();
    let shader = context2
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context2.shader_source(&shader, source);
    context2.compile_shader(&shader);

    if context2
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context2
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: Option<WebGlRenderingContext>,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let context2 = context.unwrap();

    let program = context2
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context2.attach_shader(&program, vert_shader);
    context2.attach_shader(&program, frag_shader);
    context2.link_program(&program);

    if context2
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context2
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
