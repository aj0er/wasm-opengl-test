
use std::cell::RefCell;
use std::convert::TryInto;
use std::panic;
use std::rc::Rc;

use image::ImageBuffer;
use image::ImageFormat;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn bare_bones() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct App {
    position: Vec<f32>
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {

    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        in vec4 position;

        void main() {
            gl_Position = position;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;
        uniform vec4 ourColor;
        out vec4 outColor;

        in vec2 TexCoord;

        uniform sampler2D texture1;
        
        void main() {
            outColor = texture(texture1, TexCoord);
        }
        "##,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));
    let tex = context.create_texture();
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex.unwrap()));
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT.try_into().unwrap());
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT.try_into().unwrap());

    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());

    let bytes = include_bytes!("../container.jpg");

    match image::load_from_memory_with_format(bytes, ImageFormat::Png) {
        Ok(img) => {

            // GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, data

            context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D.try_into().unwrap(), 0, WebGl2RenderingContext::RGBA.try_into().unwrap(), img.width().try_into().unwrap(), 
                img.height().try_into().unwrap(), 0, 
            WebGl2RenderingContext::RGBA.try_into().unwrap(), WebGl2RenderingContext::UNSIGNED_BYTE.try_into().unwrap(), Some(img.as_bytes())).expect("msokg");

            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D.try_into().unwrap());
            console_log!("image loaded");
        }
        Err(_) => {
            println!("input is not png");
        }
    }

    context.uniform1i(Some(&context.get_uniform_location(&program, "texture1").unwrap()), 0);

    let mut app = Rc::new(RefCell::new(App {
        position: vec![0.0, 0.0, 0.0]
    }));

    let kb_app = app.clone();

    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let mut pos = &mut kb_app.borrow_mut().position;
        (*pos)[0] = pos[0] + if event.key() == "w" {0.05} else {-0.05};        
    });

    window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();     

    main_loop(program, context, app.clone());
    Ok(())
}

fn main_callback(program: &WebGlProgram, context: &mut WebGl2RenderingContext, tick: u128, app: Rc<RefCell<App>>){
    let vertices: [f32; 32] = [
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom let
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left 
    ];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let vertexColorLocation = context.get_uniform_location(&program, "ourColor");

    let buffer = context.create_buffer().ok_or("Failed to create buffer").unwrap();
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
        .ok_or("Could not create vertex array object").unwrap();

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

    let red = (tick as f32 / 4.0).sin();
    let green = (tick as f32 / 8.0).sin();
    let blue = (tick as f32 / 16.0).sin();

    context.uniform4f(Some(&vertexColorLocation.unwrap()), red, green, blue, 1.0);
    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count);
}

fn main_loop(program: WebGlProgram, mut context: WebGl2RenderingContext, app: Rc<RefCell<App>>){
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i: u128 = 0;
    *g.borrow_mut() = Some(Closure::new(move || {
        main_callback(&program, &mut context, i, app.clone());
        i += 1;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn compile_shader(
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

pub fn link_program(
    context: &WebGl2RenderingContext,
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