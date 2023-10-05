// Uncomment these following global attributes to silence most warnings of "low" interest:

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]

extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod mesh;
mod scene_graph;
mod toolbox;

use scene_graph::SceneNode;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  pointer_to_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()


// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, normals: &Vec<f32>, colors: &Vec<f32>, indices: &Vec<u32>) -> u32 {
    // Implement me!

    // This should:
    // * Generate a VAO and bind it
    let mut arrayID = 0;
    gl::GenVertexArrays(1, &mut arrayID);
    assert!(arrayID != 0);
    gl::BindVertexArray(arrayID);
    
    // * Generate a VBO and bind it
    let mut v_bufferID = 0;
    gl::GenBuffers(1, &mut v_bufferID);
    assert!(v_bufferID != 0);
    gl::BindBuffer(gl::ARRAY_BUFFER, v_bufferID);
    // * Fill it with data
    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(vertices), pointer_to_array(vertices), gl::STATIC_DRAW);
    // println!("Size of vertices array : {}", byte_size_of_array(vertices));

    // * Configure a VAP for the data and enable it
    // Careful about stride parameter for next assignment
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const _);
    gl::EnableVertexAttribArray(0);

    // Same for the normals
    let mut n_bufferID = 0;
    gl::GenBuffers(1, &mut n_bufferID);
    assert!(n_bufferID != 0);
    gl::BindBuffer(gl::ARRAY_BUFFER, n_bufferID);
    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(normals), pointer_to_array(normals), gl::STATIC_DRAW);
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const _);
    gl::EnableVertexAttribArray(1);
    // println!("Size of normals array : {}", byte_size_of_array(normals));

    // Same for the colors
    let mut c_bufferID = 0;
    gl::GenBuffers(1, &mut c_bufferID);
    assert!(c_bufferID != 0);
    gl::BindBuffer(gl::ARRAY_BUFFER, c_bufferID);
    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(colors), pointer_to_array(colors), gl::STATIC_DRAW);
    gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, 0, 0 as *const _);
    gl::EnableVertexAttribArray(2);
    // println!("Size of colors array : {}", byte_size_of_array(colors));

    // * Generate a IBO and bind it
    let mut indexBufferID = 0;
    gl::GenBuffers(1, &mut indexBufferID);
    assert!(indexBufferID != 0);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indexBufferID);

    // * Fill it with data
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, byte_size_of_array(indices), pointer_to_array(indices), gl::STATIC_DRAW);
    // * Return the ID of the VAO*/

    arrayID
}

unsafe fn draw_scene(node: &scene_graph::SceneNode, view_projection_matrix: &glm::Mat4, transformation_so_far: &glm::Mat4) {
    // Perform any logic needed before drawing the node
    let mut model = glm::translation(&glm::vec3(-node.reference_point.x, -node.reference_point.y, -node.reference_point.z));
    model = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0)) * model;
    model = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0)) * model;
    model = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0)) * model;
    model = glm::translation(&glm::vec3(node.reference_point.x, node.reference_point.y, node.reference_point.z)) * model;
    model = glm::translation(&glm::vec3(node.position.x, node.position.y, node.position.z)) * model;
    model = transformation_so_far * model;
    let MVP_matrix = view_projection_matrix * model;

    // Check if node is drawable, if so: set uniforms, bind VAO and draw VAO
    if node.index_count != -1 {
        // Set uniforms
        gl::UniformMatrix4fv(3, 1, gl::FALSE, MVP_matrix.as_ptr());
        gl::UniformMatrix4fv(4, 1, gl::FALSE, model.as_ptr());
        // Bind VAO and draw VAO
        gl::BindVertexArray(node.vao_id);
        gl::DrawElements(gl::TRIANGLES, node.index_count, gl::UNSIGNED_INT, 0 as *const c_void);
    }

    // Recurse
    for &child in &node.children {
        draw_scene(&*child, view_projection_matrix, &model);
    }
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO around here
        let terrain = mesh::Terrain::load("./resources/lunarsurface.obj");
        let helicopter = mesh::Helicopter::load("./resources/helicopter.obj");
        let body = helicopter.body; let door = helicopter.door; let main_rotor = helicopter.main_rotor; let tail_rotor = helicopter.tail_rotor;
        
        // Create VAOs
        let (mut terrain_vao, mut body_vao, mut door_vao, mut main_rotor_vao, mut tail_rotor_vao);
        unsafe {
            terrain_vao = create_vao(&terrain.vertices, &terrain.normals, &terrain.colors, &terrain.indices);
            body_vao = create_vao(&body.vertices, &body.normals, &body.colors, &body.indices);
            door_vao = create_vao(&door.vertices, &door.normals, &door.colors, &door.indices);
            main_rotor_vao = create_vao(&main_rotor.vertices, &main_rotor.normals, &main_rotor.colors, &main_rotor.indices);
            tail_rotor_vao = create_vao(&tail_rotor.vertices, &tail_rotor.normals, &tail_rotor.colors, &tail_rotor.indices);
        };

        // Initialize nodes
        let mut scene_node = SceneNode::new();
        let terrain_node = SceneNode::from_vao(terrain_vao, terrain.indices.len() as i32);
        let mut helicopters_node = SceneNode::new();

        for i in 0..=4 {
            // New nodes
            let mut body_node = SceneNode::from_vao(body_vao, body.indices.len() as i32);
            let door_node = SceneNode::from_vao(door_vao, door.indices.len() as i32);
            let main_rotor_node = SceneNode::from_vao(main_rotor_vao, main_rotor.indices.len() as i32);
            let mut tail_rotor_node = SceneNode::from_vao(tail_rotor_vao, tail_rotor.indices.len() as i32);
            tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);

            // Add children
            body_node.add_child(&door_node);
            body_node.add_child(&main_rotor_node);
            body_node.add_child(&tail_rotor_node);

            // Add body node to helicopter parent
            helicopters_node.add_child(&body_node);
        }

        helicopters_node.print();
        
        scene_node.add_child(&terrain_node);
        scene_node.add_child(&helicopters_node);

        // scene_node.print();

        // == // Set up your shaders here

        // Basic usage of shader helper:
        // The example code below creates a 'shader' object.
        // It which contains the field `.program_id` and the method `.activate()`.
        // The `.` in the path is relative to `Cargo.toml`.
        // This snippet is not enough to do the exercise, and will need to be modified (outside
        // of just using the correct path), but it only needs to be called once

        unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
                .activate()
        };

        // Used to demonstrate keyboard handling for exercise 2.
        // let mut _arbitrary_number = 0.0; // feel free to remove
        let mut position: glm::Vec3 = glm::vec3(0.0, 0.0, -20.0);
        let z: glm::Vec3 = glm::vec3(0.0, 0.0, 1.0);
        let y: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);
        let x: glm::Vec3 = glm::vec3(1.0, 0.0, 0.0);
        let mut left_rotation = 0.0;
        let mut up_rotation = 0.0;

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }

            // Works but shorter version :
            let global_translate: glm::Mat4 = glm::translation(&position);
            let global_rotation: glm::Mat4 = glm::rotation(up_rotation, &x)
             * glm::rotation(left_rotation, &y);
            let inverse_rotation: glm::Mat4 = //glm::rotation(-up_rotation, &x)
              glm::rotation(-left_rotation, &y);
            let view = global_rotation * global_translate;
            let speed = 30.0;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html

                        // Forward
                        VirtualKeyCode::W => {
                            position += (inverse_rotation * (z.to_homogeneous() * delta_time * speed)).xyz();
                        }
                        VirtualKeyCode::S => {
                            position -= (inverse_rotation * (z.to_homogeneous() * delta_time * speed)).xyz();
                        }

                        // Left
                        VirtualKeyCode::A => {
                            position += (inverse_rotation * (x.to_homogeneous() * delta_time * speed)).xyz();
                        }
                        VirtualKeyCode::D => {
                            position -= (inverse_rotation * (x.to_homogeneous() * delta_time * speed)).xyz();
                        }

                        // Up
                        VirtualKeyCode::Space => {
                            position -= y * delta_time * speed;
                        }
                        VirtualKeyCode::LShift => {
                            position += y * delta_time * speed;
                        }

                        // left_rotation
                        VirtualKeyCode::Left => {
                            left_rotation -= delta_time;
                        }
                        VirtualKeyCode::Right => {
                            left_rotation += delta_time;
                        }

                        // up_rotation
                        VirtualKeyCode::Up => {
                            if up_rotation > - 1.57 {
                                up_rotation -= delta_time;
                            }
                        }
                        VirtualKeyCode::Down => {
                            if up_rotation < 1.57 {
                                up_rotation += delta_time;
                            }
                        }

                        // default handler:
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // Test animation for ex 3 task 3
            // body_node.position.y = (5.0 * elapsed).sin();
            // tail_rotor_node.rotation.x = 200.0 * elapsed;
            // main_rotor_node.rotation.y = 200.0 * elapsed;
            for i in 0..=4 {
                unsafe {
                    // Heading
                    let heading: toolbox::Heading = toolbox::simple_heading_animation(elapsed + (i as f32) * 1.1);
                    (*helicopters_node.children[i]).position.x = heading.x;
                    (*helicopters_node.children[i]).position.z = heading.z;
                    (*helicopters_node.children[i]).rotation.z = heading.roll;
                    (*helicopters_node.children[i]).rotation.y = heading.yaw;
                    (*helicopters_node.children[i]).rotation.x = heading.pitch;

                    // Rotors' rotation
                    (*(*helicopters_node.children[i]).children[2]).rotation.x = 200.0 * elapsed;
                    (*(*helicopters_node.children[i]).children[1]).rotation.y = 200.0 * elapsed;
                }
            }
            let perspective: glm::Mat4 = glm::perspective(window_aspect_ratio, 1.0, 1.0, 1000.0);
            let view_perspective = perspective * view;
            let model = glm::Mat4::identity();

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // == // Issue the necessary gl:: commands to draw your scene here
                draw_scene(&scene_node, &view_perspective, &model);
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
