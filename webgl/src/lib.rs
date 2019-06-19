use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::f32::consts::PI;
use nalgebra::{Matrix4, Vector3};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("output").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    // -------------------------------------
    // Create shader program and activate it

    let vert_shader = compile_shader(
        &context,
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
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        varying lowp vec4 vColor;

        void main() {
            gl_FragColor = vColor;
        }
    "#,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    
    // -------------------------------------
    // create a buffer for the wasm memory
    // somehow this is needed for converting values
    //
    // docs: get a handle to the wasm memory
    let memory_buffer = wasm_bindgen::memory()
        // try to dynamic cast it to the right type?
        .dyn_into::<WebAssembly::Memory>()?
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
    let color_buffer: web_sys::WebGlBuffer = context.create_buffer().ok_or("Failed to create color buffer.")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));

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

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;

    // bind this buffer with vertex coords, makes it "active"
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // "initializes the buffer object's data store"
    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    // now we apply the active buffer to our vertices
    let vertex_location = context.get_attrib_location(&program, "aVertexPosition");
    context.vertex_attrib_pointer_with_i32(vertex_location as u32, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(vertex_location as u32);

    // and now the same again for colors
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &colors_js_array,
        WebGlRenderingContext::STATIC_DRAW,
    );
    let color_location = context.get_attrib_location(&program, "aVertexColor");
    context.vertex_attrib_pointer_with_i32(color_location as u32, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(color_location as u32);

    // clear background
    context.clear_color(0.0, 0.0, 0.5, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    const FOV: f32 = 0.25 * PI; // field of view
    const ASPECT: f32 = 1.0; // canvas must be square

    const NEAR_Z: f32 = 0.1; // objects closer cannot be seen
    const FAR_Z: f32 = 100.0; // objects further away cannot be seen

    let projmat: Matrix4<f32> = Matrix4::new_perspective(ASPECT, FOV, NEAR_Z, FAR_Z);

    let mut mvmat: Matrix4<f32> = Matrix4::identity();
    let translation = Matrix4::new_translation(&Vector3::new(0.0, 0.0, -6.0));
    mvmat *= translation;

    let mvmat_location = context.get_uniform_location(&program, "uModelViewMatrix");
    context.uniform_matrix4fv_with_f32_array(mvmat_location.as_ref(), false, mvmat.as_slice());
    let projmat_location = context.get_uniform_location(&program, "uProjectionMatrix");
    context.uniform_matrix4fv_with_f32_array(projmat_location.as_ref(), false, projmat.as_slice());

    context.draw_arrays(
        WebGlRenderingContext::TRIANGLE_STRIP,
        0,
        (vertices.len() / 3) as i32,
    );
    Ok(())
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
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

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
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
