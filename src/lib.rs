use std::cell::RefCell;
use std::convert::TryInto;
use std::panic;
use std::rc::Rc;

use image::ImageFormat;
use na::Matrix4;
use na::Point3;
use na::Unit;
use na::UnitQuaternion;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlTexture;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

extern crate nalgebra_glm as glm;

#[macro_use]
extern crate nalgebra as na;
use na::{Vector3};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct App {
    pub camera: Camera,
    pub sample_texture: Option<WebGlTexture>,
}

struct Camera {
    pub pos: Vector3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,

    pub yaw: f32,
    pub pitch: f32,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.request_pointer_lock();

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    context.enable(WebGl2RenderingContext::DEPTH_TEST);

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
            layout (location = 0) in vec3 aPos;
            layout (location = 1) in vec2 aTexCoord;
            
            out vec2 TexCoord;
            
            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 projection;
            
            void main()
            {
                gl_Position = projection * view * model * vec4(aPos, 1.0f);
                TexCoord = vec2(aTexCoord.x, aTexCoord.y);
            }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
            precision highp float;
            out vec4 FragColor;
            in vec2 TexCoord;

            // texture sampler
            uniform sampler2D texture1;

            void main()
            {
                FragColor = texture(texture1, TexCoord);
            }
        "##,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let sample_texture: Option<WebGlTexture>;
    {
        sample_texture = context.create_texture();
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, sample_texture.as_ref());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT.try_into().unwrap());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT.try_into().unwrap());
    
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR.try_into().unwrap());
        let bytes = include_bytes!("../container.jpg");

        match image::load_from_memory_with_format(bytes, ImageFormat::Jpeg) {
            Ok(img) => {
                context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl2RenderingContext::TEXTURE_2D as u32, 0, WebGl2RenderingContext::RGBA as i32, img.width() as i32, 
                    img.height() as i32,0,WebGl2RenderingContext::RGBA,WebGl2RenderingContext::UNSIGNED_BYTE,
                    Some(&img.to_rgba8().into_vec()))?;

                context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D as u32);
                console_log!("image loaded");
            }
            Err(_) => {
                console_log!("input is not png");
            }
        }
    }    

    let app = Rc::new(RefCell::new(App {
        camera: Camera {
            pos: vector![0.0, 0.0, 3.0],
            front: vector![0.0, 0.0, -1.0],
            up: vector![0.0, 1.0, 0.0],

            yaw: -90.0,
            pitch: 0.0
        },
        sample_texture,
    }));

    let event_app = app.clone();

    let camera_speed = 0.05;
    let keyboard_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let camera = &mut event_app.borrow_mut().camera;

        match event.key().as_str() {
            "w" => {
                (*camera).pos += camera_speed * camera.front;
            }
            "s" => {
                (*camera).pos -= camera_speed * camera.front;
            }
            "a" => {
                (*camera).pos -= camera.front.cross(&camera.up).normalize() * camera_speed;
            }
            "d" => {
                (*camera).pos += camera.front.cross(&camera.up).normalize() * camera_speed;
            }
            _ => {}
        }
    });

    let mouse_app = app.clone();
    let mouse_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        if event.buttons() != 1 {
            return;
        }

        let camera = &mut mouse_app.borrow_mut().camera;

        let xoffset = event.movement_x() as f32 * 0.1;
        let yoffset = event.movement_y() as f32 * 0.1; 

        camera.yaw   += xoffset;
        camera.pitch -= yoffset;

        if camera.pitch > 89.0 {
            camera.pitch = 89.0;
        }

        if camera.pitch < -89.0 {
            camera.pitch = -89.0;
        }

        let direction: Vector3<f32> = vector![
            camera.yaw.to_radians().cos() * camera.pitch.to_radians().cos(),
            camera.pitch.to_radians().sin(), 
            camera.yaw.to_radians().sin() * camera.pitch.to_radians().cos()
        ];

        camera.front = glm::normalize(&direction);
    });

    window().add_event_listener_with_callback("keydown", keyboard_callback.as_ref().unchecked_ref()).unwrap();
    window().add_event_listener_with_callback("mousemove", mouse_callback.as_ref().unchecked_ref()).unwrap();

    keyboard_callback.forget();     
    mouse_callback.forget();

    main_loop(program, context, app.clone());
    Ok(())
}

fn main_callback(program: &WebGlProgram, context: &mut WebGl2RenderingContext, tick: u128, app_rc: Rc<RefCell<App>>){
    let app = app_rc.borrow();
    let cube_positions = [
        vector![ 0.0,  0.0,  0.0], 
        /*
        vector![ 2.0,  5.0, -15.0], 
        vector![-1.5, -2.2, -2.5],  
        vector![-3.8, -2.0, -12.3],  
        vector![ 2.4, -0.4, -3.5],  
        vector![-1.7,  3.0, -7.5],  
        vector![ 1.3, -2.0, -2.5],  
        vector![ 1.5,  2.0, -2.5], 
        vector![ 1.5,  0.2, -1.5], 
        vector![-1.3,  1.0, -1.5]  
        */
    ];

    let vertices = [
       -0.5, -0.5, -0.5,  0.0, 0.0,
        0.5, -0.5, -0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
       -0.5,  0.5, -0.5,  0.0, 1.0,
       -0.5, -0.5, -0.5,  0.0, 0.0,
   
       -0.5, -0.5,  0.5,  0.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
       -0.5,  0.5,  0.5,  0.0, 1.0,
       -0.5, -0.5,  0.5,  0.0, 0.0,
   
       -0.5,  0.5,  0.5,  1.0, 0.0,
       -0.5,  0.5, -0.5,  1.0, 1.0,
       -0.5, -0.5, -0.5,  0.0, 1.0,
       -0.5, -0.5, -0.5,  0.0, 1.0,
       -0.5, -0.5,  0.5,  0.0, 0.0,
       -0.5,  0.5,  0.5,  1.0, 0.0,
   
        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5,  0.5,  0.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
   
       -0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  1.0, 1.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
       -0.5, -0.5,  0.5,  0.0, 0.0,
       -0.5, -0.5, -0.5,  0.0, 1.0,
   
       -0.5,  0.5, -0.5,  0.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
       -0.5,  0.5,  0.5,  0.0, 0.0,
       -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    let position_attribute_location = context.get_attrib_location(&program, "aPos");
    let tex_attribute_location = context.get_attrib_location(&program, "aTexCoord");

    // VAO ------------------------
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object").unwrap();

    context.bind_vertex_array(Some(&vao));

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

    // position
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        5 * 4,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    // tex coord
    context.vertex_attrib_pointer_with_i32(
        tex_attribute_location as u32,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        5 * 4,
        3 * 4,
    );
    context.enable_vertex_attrib_array(tex_attribute_location as u32);

    let mut model = Matrix4::from_diagonal_element(1.0);
    model = UnitQuaternion::from_axis_angle(&Unit::new_unchecked(Vector3::new(0.5, 1.0, 0.0)), -55.0_f32.to_radians() * (tick as f32 / 30.0)).to_homogeneous() * model;

    let mut view = Matrix4::look_at_rh(&Point3::from(app.camera.pos), &Point3::from(app.camera.pos + app.camera.front), &app.camera.up);

    /*
    let mut view = Matrix4::from_diagonal_element(1.0);*/
    //view = view.prepend_translation(&vector![app.position[0], 0.0, app.position[2]]);

    let projection = glm::perspective(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);

    let model_loc = context.get_uniform_location(program, "model");
    context.uniform_matrix4fv_with_f32_array(model_loc.as_ref(), false, model.data.as_slice());

    let view_loc = context.get_uniform_location(program, "view");
    context.uniform_matrix4fv_with_f32_array(view_loc.as_ref(), false, view.data.as_slice());

    let projection_loc = context.get_uniform_location(program, "projection");
    context.uniform_matrix4fv_with_f32_array(projection_loc.as_ref(), false, projection.data.as_slice());

    context.clear_color(0.0, 0.0, 1.589, 0.5);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, app.sample_texture.as_ref());
    context.bind_vertex_array(Some(&vao));

    for i in 0..cube_positions.len() {
        let translation = cube_positions[i];
        let mut model = Matrix4::from_diagonal_element(1.0);
        model = model.prepend_translation(&translation);
        //model = UnitQuaternion::from_axis_angle(&Unit::new_unchecked(Vector3::new(0.0, 0.5, 0.0)), -55.0_f32.to_radians() * (tick as f32 / 30.0 as f32)).to_homogeneous() * model;

        let model_loc = context.get_uniform_location(program, "model");
        context.uniform_matrix4fv_with_f32_array(model_loc.as_ref(), false, model.data.as_slice());

        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 36, /*WebGl2RenderingContext::UNSIGNED_INT, 0*/);
    }
}

fn main_loop(program: WebGlProgram, mut context: WebGl2RenderingContext, app: Rc<RefCell<App>>){
    let first = Rc::new(RefCell::new(None));
    let second = first.clone();

    let mut i: u128 = 0;
    *second.borrow_mut() = Some(Closure::new(move || {
        main_callback(&program, &mut context, i, app.clone());
        i += 1;
        request_animation_frame(first.borrow().as_ref().unwrap());
    }));

    request_animation_frame(second.borrow().as_ref().unwrap());
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
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