
use std::cell::RefCell;
use std::convert::TryInto;
<<<<<<< HEAD
use std::rc::Rc;

use image::ImageFormat;
use na::Matrix4;
use na::Quaternion;
use na::Rotation;
=======
use std::panic;
use std::rc::Rc;

use image::ImageBuffer;
use image::ImageFormat;
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlTexture;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

<<<<<<< HEAD
extern crate nalgebra_glm as glm;

#[macro_use]
extern crate nalgebra as na;
use na::{Vector3, Rotation3};
=======
extern crate console_error_panic_hook;
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781

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
    pub position: Vec<f32>,
    pub tex1: Option<WebGlTexture>,
<<<<<<< HEAD
=======
    pub tex2: Option<WebGlTexture>
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
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
<<<<<<< HEAD
            layout (location = 0) in vec3 aPos;
            layout (location = 1) in vec3 aColor;
            layout (location = 2) in vec2 aTexCoord;

            out vec3 ourColor;
            out vec2 TexCoord;

            uniform mat4 transform;

            void main()
            {
                gl_Position = transform * vec4(aPos, 1.0f);
                ourColor = aColor;
                TexCoord = vec2(aTexCoord.x, aTexCoord.y);
            } 
=======
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;
        layout (location = 2) in vec2 aTexCoord;
        
        out vec3 ourColor;
        out vec2 TexCoord;
        
        void main()
        {
            gl_Position = vec4(aPos, 1.0);
            ourColor = aColor;
            TexCoord = vec2(aTexCoord.x, aTexCoord.y);
        }
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
<<<<<<< HEAD
            precision highp float;
            out vec4 FragColor;
            in vec3 ourColor;
            in vec2 TexCoord;

            // texture sampler
            uniform sampler2D texture1;

            void main()
            {
                FragColor = texture(texture1, TexCoord) * vec4(ourColor, 1.0);
            }
=======
    
        precision highp float;
        uniform vec4 ourColor;
        out vec4 outColor;

        in vec2 TexCoord;

        // texture samplers
        uniform sampler2D texture1;
        uniform sampler2D texture2;

        void main()
        {
            outColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.2);
        }
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
        "##,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let mut tex1: Option<WebGlTexture> = None;
<<<<<<< HEAD
    {
        tex1 = context.create_texture();
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, tex1.as_ref());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT.try_into().unwrap());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT.try_into().unwrap());
    
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
=======
    let mut tex2: Option<WebGlTexture> = None;

    {
    tex1 = context.create_texture();
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, tex1.as_ref());
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT.try_into().unwrap());
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT.try_into().unwrap());

    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());

    
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
        let bytes = include_bytes!("../container.jpg");

        match image::load_from_memory_with_format(bytes, ImageFormat::Jpeg) {
            Ok(img) => {
<<<<<<< HEAD
                context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl2RenderingContext::TEXTURE_2D as u32, 0, WebGl2RenderingContext::RGBA as i32, img.width() as i32, 
                    img.height() as i32,0,WebGl2RenderingContext::RGBA,WebGl2RenderingContext::UNSIGNED_BYTE,
                    Some(&img.to_rgba8().into_vec()))?;

                context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D as u32);
=======

                // GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, data

                context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl2RenderingContext::TEXTURE_2D.try_into().unwrap(), 0, WebGl2RenderingContext::RGBA.try_into().unwrap(), img.width().try_into().unwrap(), 
                    img.height().try_into().unwrap(), 0, 
                WebGl2RenderingContext::RGBA.try_into().unwrap(), WebGl2RenderingContext::UNSIGNED_BYTE.try_into().unwrap(), Some(img.as_bytes())).expect("msokg");

                context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D.try_into().unwrap());
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
                console_log!("image loaded");
            }
            Err(_) => {
                console_log!("input is not png");
            }
        }
<<<<<<< HEAD
    }    
=======
    }

    {
        tex2 = context.create_texture();
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, tex2.as_ref());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT.try_into().unwrap());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT.try_into().unwrap());
    
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
    
        
            let bytes = include_bytes!("../awesomeface.png");
    
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
                    console_log!("input is not png");
                }
            }
        }

    context.uniform1i(Some(&context.get_uniform_location(&program, "texture1").unwrap()), 0);
    context.uniform1i(Some(&context.get_uniform_location(&program, "texture2").unwrap()), 1);
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781

    let mut app = Rc::new(RefCell::new(App {
        position: vec![0.0, 0.0, 0.0],
        tex1,
<<<<<<< HEAD
=======
        tex2
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
    }));

    let kb_app = app.clone();

    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let mut pos = &mut kb_app.borrow_mut().position;

        match event.key().as_str() {
            "w" => {
                (*pos)[2] += 0.05;
            }
            "s" => {
                (*pos)[2] -= 0.05;
            }
            "a" => {
                (*pos)[0] += 0.05;
            }
            "d" => {
                (*pos)[0] -= 0.05;
            }
            _ => {}
        }
    });

    window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();     

    main_loop(program, context, app.clone());
    Ok(())
}

fn main_callback(program: &WebGlProgram, context: &mut WebGl2RenderingContext, tick: u128, app: Rc<RefCell<App>>){
    let aaa = app.borrow();
    let vertices: [f32; 32] = [
<<<<<<< HEAD
        // positions                                       // colors           // texture coords
        aaa.position[0],  0.5, 0.0,            1.0, 0.0, 0.0,      1.0, 1.0, // top right
        aaa.position[0], -0.5, 0.0,            0.0, 1.0, 0.0,      1.0, 0.0, // bottom right
       -aaa.position[0], -0.5, 0.0,            0.0, 0.0, 1.0,      0.0, 0.0, // bottom let
       -aaa.position[0],  0.5, 0.0,            1.0, 1.0, 0.0,      0.0, 1.0  // top let 
    ];

    let indices: [u32; 6] = [
        0, 1, 3, // first triangle
        1, 2, 3  // second triangle
=======
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom let
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left 
    ];

    let indives: [f32; 6] = [
        0.0, 1.0, 3.0, // first triangle
        1.0, 2.0, 3.0  // second triangle
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
    ];

    let position_attribute_location = context.get_attrib_location(&program, "aPos");
    let color_attribute_location = context.get_attrib_location(&program, "aColor");
    let tex_attribute_location = context.get_attrib_location(&program, "aTexCoord");

<<<<<<< HEAD
=======
    //let vertexColorLocation = context.get_uniform_location(&program, "ourColor");

>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
    // VBO --------------------
    let vbo = context.create_buffer().ok_or("Failed to create buffer").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    // EBO ----------------------------
    let ebo = context.create_buffer().ok_or("Failed to create buffer").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));
    unsafe {
<<<<<<< HEAD
        let positions_array_buf_view = js_sys::Uint32Array::view(&indices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
=======
        let positions_array_buf_view = js_sys::Float32Array::view(&indives);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    // VAO ------------------------
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object").unwrap();

    context.bind_vertex_array(Some(&vao));

     // position attribute
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        8 * 4,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);
    // color attribute
    context.vertex_attrib_pointer_with_i32(
        color_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        8 * 4,
        3 * 4,
    );
    context.enable_vertex_attrib_array(color_attribute_location as u32);

    // tex coord attribute
    context.vertex_attrib_pointer_with_i32(
        tex_attribute_location as u32,
<<<<<<< HEAD
        2,
=======
        3,
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
        WebGl2RenderingContext::FLOAT,
        false,
        8 * 4,
        6 * 4,
    );
<<<<<<< HEAD
    context.enable_vertex_attrib_array(tex_attribute_location as u32); 

    let mut trans = Matrix4::from_diagonal_element(1.0);
    trans = Rotation::from_axis_angle(&Vector3::z_axis(), (tick as f32).to_radians()).to_homogeneous() * trans;
    trans = trans.prepend_nonuniform_scaling(&Vector3::new(0.5, 0.5, 0.5));

    let transform_loc = context.get_uniform_location(program, "transform");
    context.uniform_matrix4fv_with_f32_array(transform_loc.as_ref(), false, trans.data.as_slice());

    context.clear_color(0.0, 0.0, 1.589, 0.5);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, aaa.tex1.as_ref());
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));
    context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_INT, 0);

=======
    context.enable_vertex_attrib_array(tex_attribute_location as u32);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.active_texture(WebGl2RenderingContext::TEXTURE0);
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, aaa.tex1.as_ref());

    context.active_texture(WebGl2RenderingContext::TEXTURE1);
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, aaa.tex2.as_ref());

    /* 
    let red = (tick as f32 / 4.0).sin();
    let green = (tick as f32 / 8.0).sin();
    let blue = (tick as f32 / 16.0).sin();*/

    context.bind_vertex_array(Some(&vao));
    context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 4, WebGl2RenderingContext::UNSIGNED_INT, 0);

    //context.uniform4f(Some(&vertexColorLocation.unwrap()), red, green, blue, 1.0);
>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
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

<<<<<<< HEAD
=======
fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 3, WebGl2RenderingContext::UNSIGNED_INT, 0);
}

>>>>>>> 325e32a44405e9ef1de28d29bffebd91e2120781
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