use std::cell::RefCell;

use std::panic;

use std::rc::Rc;

use image::ImageFormat;
use na::Matrix4;
use na::Point3;
use na::Rotation;
use na::Rotation3;

use shader::Shader;
use texture::load_texture;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use web_sys::WebGlTexture;
use web_sys::{WebGl2RenderingContext};

extern crate nalgebra_glm as glm;

pub mod shader;
pub mod texture;
pub mod utils;

#[macro_use]
extern crate nalgebra as na;
use na::Vector3;

struct App {
    pub camera: Camera,
    pub world: World,
    pub sample_texture: Option<WebGlTexture>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct Camera {
    pub pos: Vector3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,

    pub yaw: f32,
    pub pitch: f32,
}

struct World {
    pub cubes: Vec<Cube>
}

const CAMERA_SPEED: f32 = 0.1;

const CUBE_MODEL: [f32; 180] = [
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

struct Cube {
    translation: Vector3<f32>,
    rotation: Vector3<f32>
}

impl Cube {
    fn get_rotated_translation(&self) -> Rotation<f32, 3> {
        return Rotation3::new(self.rotation);
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let world = load_world();
    canvas.request_pointer_lock();

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    context.enable(WebGl2RenderingContext::DEPTH_TEST);

    let shader = Shader::new(&context, include_str!("../vertex.glsl"), include_str!("../fragment.glsl"));
    context.use_program(Some(&shader.program));

    let sample_texture: Option<WebGlTexture> = Some(load_texture(&context, include_bytes!("../container.jpg"), ImageFormat::Jpeg));

    let app = Rc::new(RefCell::new(App {
        camera: Camera {
            pos: vector![0.0, 0.0, 3.0],
            front: vector![0.0, 0.0, -1.0],
            up: vector![0.0, 1.0, 0.0],

            yaw: -90.0,
            pitch: 0.0
        },
        world,
        sample_texture,
    }));

    let event_app = app.clone();
    let keyboard_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        on_key(event, &mut event_app.borrow_mut());
    });

    let mouse_app = app.clone();
    let mouse_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        on_mouse(event, &mut mouse_app.borrow_mut().camera);
    });

    window().add_event_listener_with_callback("keydown", keyboard_callback.as_ref().unchecked_ref()).unwrap();
    window().add_event_listener_with_callback("mousemove", mouse_callback.as_ref().unchecked_ref()).unwrap();

    keyboard_callback.forget();     
    mouse_callback.forget();

    start_main_loop(shader, context, app.clone());
    Ok(())
}

fn load_world() -> World {
    return World {
        cubes: vec![
            Cube {
               translation: vector![0.0, 2.0, 0.0],
               rotation: vector![0.0, std::f32::consts::PI / 4.0, 1.0]
            }
        ]
    };
}

fn on_mouse(event: MouseEvent, camera: &mut Camera){
    if event.buttons() != 1 { // Vänstra musknappen måste vara nere
        return;
    }

    let x_offset = event.movement_x() as f32 * 0.1;
    let y_offset = event.movement_y() as f32 * 0.1; 

    camera.yaw   += x_offset;
    camera.pitch -= y_offset;

    // Förhindra att spelaren vrider nacken av sig
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
}

fn on_key(event: KeyboardEvent, app: &mut App){
    let camera = &mut app.camera;

    match event.key().as_str() {
        "w" => {
            (*camera).pos += CAMERA_SPEED * camera.front;
        }
        "s" => {
            (*camera).pos -= CAMERA_SPEED * camera.front;
        }
        "a" => {
            (*camera).pos -= camera.front.cross(&camera.up).normalize() * CAMERA_SPEED;
        }
        "d" => {
            (*camera).pos += camera.front.cross(&camera.up).normalize() * CAMERA_SPEED;
        }
        " " => {
            (*camera).pos += CAMERA_SPEED * camera.up;
        }
        "Shift" => {
            (*camera).pos -= CAMERA_SPEED * camera.up;
        }
        "e" => {

            /*
            let direction: Vector3<f32> = vector![
                camera.yaw.to_radians().cos() * camera.pitch.to_radians().cos(),
                camera.pitch.to_radians().sin(), 
                camera.yaw.to_radians().sin() * camera.pitch.to_radians().cos()
            ];

            let rot = Rotation3::new(direction);
            let _transformed = rot.transform_vector(&vector![0.0, 0.0, -3.0]);*/

            let cube = Cube {
                translation: camera.pos + vector![0.0, 0.0, -3.0],
                rotation: vector![0.0, 0.0, 0.0]
            };

            (*app).world.cubes.push(cube);
        }
        _ => {}
    }
}

fn draw(shader: &Shader, context: &mut WebGl2RenderingContext, _tick: u128, app_rc: Rc<RefCell<App>>){
    let app = app_rc.borrow();

    let position_attribute_location = context.get_attrib_location(&shader.program, "aPos");
    let tex_attribute_location = context.get_attrib_location(&shader.program, "aTexCoord");

    // VAO ------------------------
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object").unwrap();

    context.bind_vertex_array(Some(&vao));

    // VBO --------------------
    let vbo = context.create_buffer().ok_or("Failed to create buffer").unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&CUBE_MODEL);

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

    /* 
    let mut model = Matrix4::from_diagonal_element(1.0);
    model = UnitQuaternion::from_axis_angle(&Unit::new_unchecked(Vector3::new(0.5, 1.0, 0.0)), 
        -55.0_f32.to_radians() * (tick as f32 / 30.0)).to_homogeneous() * model;*/

    let view = Matrix4::look_at_rh(&Point3::from(app.camera.pos), 
    &Point3::from(app.camera.pos + app.camera.front), &app.camera.up);

    let projection = glm::perspective(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);

    /*
    let model_loc = context.get_uniform_location(&shader.program, "model");
    context.uniform_matrix4fv_with_f32_array(model_loc.as_ref(), false, model.data.as_slice()); */

    let view_loc = context.get_uniform_location(&shader.program, "view");
    context.uniform_matrix4fv_with_f32_array(view_loc.as_ref(), false, view.data.as_slice());

    let projection_loc = context.get_uniform_location(&shader.program, "projection");
    context.uniform_matrix4fv_with_f32_array(projection_loc.as_ref(), false, projection.data.as_slice());

    context.clear_color(0.0, 0.0, 1.589, 0.4);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, app.sample_texture.as_ref());
    context.bind_vertex_array(Some(&vao));

    for i in 0..app.world.cubes.len() {
        let translation = app.world.cubes[i].translation;
        let mut model = Matrix4::from_diagonal_element(1.0);
        model = model.prepend_translation(&translation);
        //model = model * Rotation3::new(vector![0.0, 0.0, (tick as f32) / 30.0]).to_homogeneous();
        model = model * app.world.cubes[i].get_rotated_translation().to_homogeneous();

        let model_loc = context.get_uniform_location(&shader.program, "model");
        context.uniform_matrix4fv_with_f32_array(model_loc.as_ref(), false, model.data.as_slice());

        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 36); // 36 vertices per kub 
    }
}

fn start_main_loop(shader: Shader, mut context: WebGl2RenderingContext, app: Rc<RefCell<App>>){
    let first = Rc::new(RefCell::new(None));
    let second = first.clone();

    let mut tick: u128 = 0;
    *second.borrow_mut() = Some(Closure::new(move || {
        draw(&shader, &mut context, tick, app.clone());
        tick += 1;
        request_animation_frame(first.borrow().as_ref().unwrap());
    }));

    request_animation_frame(second.borrow().as_ref().unwrap());
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(callback: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}