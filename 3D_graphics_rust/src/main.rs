extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod mesh;
mod toolbox;
mod scene_graph;
use scene_graph::SceneNode;


use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 1000;
const SCREEN_H: u32 = 700;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You WILL need these! // == //
// The names should be pretty self explanatory
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()


// == // Modify and complete the function below for the first task
unsafe fn VAO_setup(coords: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>, normals: &Vec<f32>) -> u32 {
    // Create and bind VAO
    let mut vao_id = 0;
    gl::GenVertexArrays(1, &mut vao_id);
    gl::BindVertexArray(vao_id);
    println!("VAO is loaded: {}. It has ID: {}", gl::GenVertexArrays::is_loaded(), vao_id);

    // Combine coordinates, colors and normals. Combined format is now: [x,y,z,r,g,b,i,j,k, x,y,x...]
    let mut combined: Vec<f32> = vec![];
    let length_divided_by_three = coords.len()/3;
    for i in 0..length_divided_by_three {
        for j in 0..3{
            combined.push(coords[i*3+j]);
        }
        for j in 0..4{
            combined.push(colors[i*4+j]);
        }
        for j in 0..3{
            combined.push(normals[i*3+j]);
        }
    }

    // Create, bind, fill and unbind VBO
    let mut vbo_id = 0;
    gl::GenBuffers(1, &mut vbo_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array::<f32>(&combined), pointer_to_array::<f32>(&combined), gl::STATIC_DRAW);
    //?gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    println!("VBO is loaded: {}. It has ID: {}", gl::GenBuffers::is_loaded(), vbo_id);
    
    // Bytes in float * amount of floats per attribute(vertices, colors, normals) * amount of attributes + 4 because alpha
    let stride = 4*3*3+4;
    // VertexAttrib for vertices
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);

    // VertexAttrib for colors
    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, stride, offset::<f32>(3));
    gl::EnableVertexAttribArray(1);

    // VertexAttrib for normals
    gl::VertexAttribPointer(4, 3, gl::FLOAT, gl::FALSE, stride, offset::<f32>(7));
    gl::EnableVertexAttribArray(4);

    // Index buffer
    let mut ind_buf_id = 0;
    gl::GenBuffers(1, &mut ind_buf_id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ind_buf_id);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, byte_size_of_array::<u32>(indices), pointer_to_array::<u32>(indices), gl::STATIC_DRAW);

    vao_id
} 

unsafe fn update_node_transformations(node: &mut scene_graph::SceneNode, transformation_so_far: &glm::Mat4) {
    // Construct the correct transformation matrix
    // Translate to reference point
    let translation_to_ref = glm::translation(&-node.reference_point);
    // Rotate around x
    let rotate_x = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 
        0.0, node.rotation[0].cos(), -node.rotation[0].sin(), 0.0, 
        0.0, node.rotation[0].sin(), node.rotation[0].cos(), 0.0, 
        0.0, 0.0, 0.0, 1.0 
    );
    // Rotate around y
    let rotate_y = glm::mat4(
        node.rotation[1].cos(), 0.0, node.rotation[1].sin(), 0.0, 
        0.0, 1.0, 0.0, 0.0, 
        -node.rotation[1].sin(), 0.0, node.rotation[1].cos(), 0.0, 
        0.0, 0.0, 0.0, 1.0 
    );
    // Rotate around z
    let rotate_z = glm::mat4(
        node.rotation[2].cos(), -node.rotation[2].sin(), 0.0, 0.0, 
        node.rotation[2].sin(), node.rotation[2].cos(), 0.0, 0.0, 
        0.0, 0.0, 1.0, 0.0, 
        0.0, 0.0, 0.0, 1.0 
    );
    // Translate back
    let translation_back = glm::translation(&node.reference_point);

    let translate_position = glm::translation(&glm::vec3(node.position[0], node.position[1], node.position[2]));
    
    //println!("Rotation of node with {} children is {} {}", node.children.len(), node.rotation, node.rotation[0]);
    // Update the node's transformation matrix
    node.current_transformation_matrix = transformation_so_far * translate_position * translation_back * rotate_x * rotate_y * rotate_z * translation_to_ref;
    
    // Recurse
    for &child in &node.children {
        update_node_transformations(&mut *child, &node.current_transformation_matrix);
    }
}

unsafe fn draw_scene(node: &scene_graph::SceneNode, view_projection_matrix: &glm::Mat4) {
    // Check if node is drawable, bind vao_id and draw
    if node.index_count > 0 {
        gl::UniformMatrix4fv(2, 1, gl::FALSE, (view_projection_matrix * node.current_transformation_matrix).as_ptr());
        gl::UniformMatrix4fv(5, 1, gl::FALSE, node.current_transformation_matrix.as_ptr());
        gl::BindVertexArray(node.vao_id);
        gl::DrawElements(gl::TRIANGLES, node.index_count, gl::UNSIGNED_INT, ptr::null());
        //println!("Ref point for node with {} children is {}", node.children.len(), node.reference_point);
    }
    // Call draw_scene for each child node aswell
    for &child in &node.children {
        draw_scene(&*child, view_projection_matrix);
    }
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("TDT4195")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            // Disabled face culling to allow us to see the triangles from the back aswell
            //gl::Enable(gl::CULL_FACE);
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

        //let vao_id = unsafe { VAO_setup(&vertices, &indices, &colors) };
        // Load meshes
        let lunar_surface = mesh::Terrain::load("./resources/lunarsurface.obj");
        let helicopter = mesh::Helicopter::load("./resources/helicopter.obj");
        
        let mut root_node = SceneNode::new();
        const n_of_helicopters: usize = 5;
        let mut movement_speed = 1.0;

        let lunar_vao_id = unsafe { VAO_setup(&lunar_surface.vertices, &lunar_surface.indices, &lunar_surface.colors, &lunar_surface.normals) };
        let mut lunar_node = SceneNode::from_vao(lunar_vao_id, lunar_surface.indices.len() as i32);
        
        let mut body_nodes: Vec<scene_graph::Node> = Vec::new();
        for i in 0..n_of_helicopters {
            let helicopter_body_vao_id = unsafe { VAO_setup(&helicopter.body.vertices, &helicopter.body.indices, &helicopter.body.colors, &helicopter.body.normals)};
            let mut helicopter_body_node = SceneNode::from_vao(helicopter_body_vao_id, helicopter.body.indices.len() as i32);

            let helicopter_body_vao_id = unsafe { VAO_setup(&helicopter.body.vertices, &helicopter.body.indices, &helicopter.body.colors, &helicopter.body.normals)};
            let mut helicopter_body_node = SceneNode::from_vao(helicopter_body_vao_id, helicopter.body.indices.len() as i32);
            
            let helicopter_door_vao_id = unsafe { VAO_setup(&helicopter.door.vertices, &helicopter.door.indices, &helicopter.door.colors, &helicopter.door.normals)};
            let mut helicopter_door_node = SceneNode::from_vao(helicopter_door_vao_id, helicopter.door.indices.len() as i32);
            
            let helicopter_main_rotor_vao_id = unsafe { VAO_setup(&helicopter.main_rotor.vertices, &helicopter.main_rotor.indices, &helicopter.main_rotor.colors, &helicopter.main_rotor.normals)};
            let mut helicopter_main_rotor_node = SceneNode::from_vao(helicopter_main_rotor_vao_id, helicopter.main_rotor.indices.len() as i32);
            
            let helicopter_tail_rotor_vao_id = unsafe { VAO_setup(&helicopter.tail_rotor.vertices, &helicopter.tail_rotor.indices, &helicopter.tail_rotor.colors, &helicopter.tail_rotor.normals)};
            let mut helicopter_tail_rotor_node = SceneNode::from_vao(helicopter_tail_rotor_vao_id, helicopter.tail_rotor.indices.len() as i32);
    
            helicopter_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
            // Add relationships between nodes
            helicopter_body_node.add_child(&helicopter_door_node);
            helicopter_body_node.add_child(&helicopter_main_rotor_node);
            helicopter_body_node.add_child(&helicopter_tail_rotor_node);
            lunar_node.add_child(&helicopter_body_node);
    
            body_nodes.push(helicopter_body_node);
        }
                
        root_node.add_child(&lunar_node);

        let prog_id = unsafe {
            let shader = shader::ShaderBuilder::new()
            // Gets shader files and compiles them
            .attach_file("./shaders/simple.frag")
            .attach_file("./shaders/simple.vert")
            // Links shaders
            .link();
            // Runs UseProgram with the program ID, sort of an unnecessary function imo
            shader.activate();
            // Return the program id of the shader
            shader.program_id
        };
        

        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;

        // Camera orientation and position variables
        let mut camera_x: f32 = 0.0;
        let mut camera_y: f32 = 0.0;
        let mut camera_z: f32 = -2.0; // Initialized to -2 to do the initial translation
        let mut camera_horizontal_rot: f32 = 0.0;
        let mut camera_vertical_rot: f32 = 0.0;
        let scaling_factor: f32 = 50.0;
        let rotation_scaling_factor: f32 = 2.0;
        // The main rendering loop
        let mut counter = 0;
        let perspective: glm::Mat4 = glm::perspective(SCREEN_W as f32 / SCREEN_H as f32, 1.2, 1.0, 1000.0);

        // An array of bools that contains information about the state of each helicopters door
        let mut opening_door: [bool; n_of_helicopters] = [false; n_of_helicopters];
        let mut time_of_last_door_state_change = std::time::Instant::now();

        loop {
            counter+=1;
            if(counter % 100 == 0){
                //println!("xyz: ({}, {}, {}) | Horizontal angle: {}, sin: {}, cos: {} | | Vertical angle: {}, sin: {}, cos: {} |", camera_x, camera_y,camera_z, camera_horizontal_rot, camera_horizontal_rot.sin(), camera_horizontal_rot.cos(), camera_vertical_rot, camera_vertical_rot.sin(), camera_vertical_rot.cos());
            }
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // x and z movement is controlled by WASD, y movement with space and left contro. Rotation with arrow keys
                        // Since the rotation is also flipped (panning left corresponds to decreasing angle) we multiply the angle by -1
                        VirtualKeyCode::W => {
                            // We want to move straight ahead in the direction we're facing
                            camera_z += delta_time*scaling_factor*(-1.0*camera_horizontal_rot).cos();
                            camera_x += delta_time*scaling_factor*(-1.0*camera_horizontal_rot).sin();
                            camera_y += delta_time*scaling_factor*camera_vertical_rot.sin();
                        },
                        VirtualKeyCode::A => {
                            // We want to move in the direction 90 degrees (or PI/2) to the left of where we're facing
                            camera_z += delta_time*scaling_factor*(-1.0*(camera_horizontal_rot - std::f32::consts::PI/2.0)).cos();
                            camera_x += delta_time*scaling_factor*(-1.0*(camera_horizontal_rot - std::f32::consts::PI/2.0)).sin();
                        },
                        VirtualKeyCode::S => {
                            // We want to move in the direction 180 degrees (or 1 PI) from where we're facing
                            camera_z += delta_time*scaling_factor*(-1.0*camera_horizontal_rot + std::f32::consts::PI).cos();
                            camera_x += delta_time*scaling_factor*(-1.0*camera_horizontal_rot + std::f32::consts::PI).sin();
                            camera_y += delta_time*scaling_factor*(camera_vertical_rot + std::f32::consts::PI).sin();
                            
                        },
                        VirtualKeyCode::D => {
                            // We want to move in the direction 90 degrees (or PI/2) to the right of where we're facing
                            camera_z += delta_time*scaling_factor*(-1.0*(camera_horizontal_rot + std::f32::consts::PI/2.0)).cos();
                            camera_x += delta_time*scaling_factor*(-1.0*(camera_horizontal_rot + std::f32::consts::PI/2.0)).sin();
                        },
                        // y controlled by space and ctrl
                        // Honestly not 100% sure about these, but i think it works as intended, it becomes hard to conseptualize after a while
                        VirtualKeyCode::Space => {
                            //camera_z -= delta_time*scaling_factor*(-camera_horizontal_rot).cos()*(-camera_vertical_rot).sin();
                            //camera_x -= delta_time*scaling_factor*(-camera_horizontal_rot).sin()*(-camera_vertical_rot).sin();
                            camera_y -= delta_time*scaling_factor;//*(-1.0*camera_vertical_rot).cos();
                        },
                        VirtualKeyCode::LControl=> {
                            //camera_z += delta_time*scaling_factor*(-camera_horizontal_rot).cos()*(-camera_vertical_rot).sin();
                            //camera_x += delta_time*scaling_factor*(-camera_horizontal_rot).sin()*(-camera_vertical_rot).sin();
                            camera_y += delta_time*scaling_factor;//*(-1.0*camera_vertical_rot).cos();
                        },

                        // Rotation horizontal and vertical controlled by arrow keys
                        VirtualKeyCode::Left => {
                            camera_horizontal_rot -= delta_time*rotation_scaling_factor;
                        },
                        VirtualKeyCode::Right => {
                            camera_horizontal_rot += delta_time*rotation_scaling_factor;
                        },
                        VirtualKeyCode::Up => {
                            camera_vertical_rot -= delta_time*rotation_scaling_factor;
                        },
                        VirtualKeyCode::Down => {
                            camera_vertical_rot += delta_time*rotation_scaling_factor;
                        },
                        // Reset camera position and rotation
                        VirtualKeyCode::R => {
                            camera_z = -2.0;
                            camera_x = 0.0;
                            camera_y = 0.0;
                            camera_horizontal_rot = 0.0;
                            camera_vertical_rot = 0.0;

                        },
                        // Preset for seeing the differece between noperspective and smooth
                        VirtualKeyCode::P => {
                            camera_x = 17.518263;
                            camera_y = -2.1809983;
                            camera_z = -0.0000011129528;
                            camera_horizontal_rot = 1.4908568;
                            camera_vertical_rot = 0.0;
                        },
                        // Open door
                        VirtualKeyCode::O => {
                            unsafe{
                                time_of_last_door_state_change = std::time::Instant::now();
                                for i in 0..body_nodes.len() {
                                    // Close
                                    opening_door[i] = true;
                                }     
                            }
                        },
                        // Close door
                        VirtualKeyCode::I => {
                            unsafe{
                                time_of_last_door_state_change = std::time::Instant::now();
                                for i in 0..body_nodes.len() {
                                    // Close
                                    opening_door[i] = false;
                                }     
                            }
                        },
                        VirtualKeyCode::N => {
                            movement_speed = movement_speed * 1.05;
                        },
                        VirtualKeyCode::M => {
                            movement_speed = movement_speed * 0.95;
                        },
                        
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                *delta = (0.0, 0.0);
            }

            unsafe {
                gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                
                let translate_by_camera_pos: glm::Mat4 = glm::mat4(
                    1.0, 0.0, 0.0, camera_x, 
                    0.0, 1.0, 0.0, camera_y, 
                    0.0, 0.0, 1.0, camera_z, 
                    0.0, 0.0, 0.0, 1.0 
                );

                let cos_theta = camera_horizontal_rot.cos();
                let sin_theta = camera_horizontal_rot.sin();

                let horizontal_rotation: glm::Mat4 = glm::mat4(
                    cos_theta, 0.0, sin_theta, 0.0, 
                    0.0, 1.0, 0.0, 0.0, 
                    -sin_theta, 0.0, cos_theta, 0.0, 
                    0.0, 0.0, 0.0, 1.0 
                );

                let cos_theta_vert = camera_vertical_rot.cos();
                let sin_theta_vert = camera_vertical_rot.sin();
                
                let vertical_rotation: glm::Mat4 = glm::mat4(
                    1.0, 0.0, 0.0, 0.0, 
                    0.0, cos_theta_vert, -sin_theta_vert, 0.0, 
                    0.0, sin_theta_vert, cos_theta_vert, 0.0, 
                    0.0, 0.0, 0.0, 1.0 
                );

                let combined_transformation =  perspective*vertical_rotation*horizontal_rotation*translate_by_camera_pos;

                let path = toolbox::simple_heading_animation(elapsed * movement_speed);
                //println!("Door is opening: {}", opening_door[0]);
                for i in 0..body_nodes.len() {
                    body_nodes[i].position = glm::vec3(path.x + (40*i) as f32, elapsed.sin() * 10.0 + (20*(i+1)) as f32, path.z);
                    //body_nodes[i].position = glm::vec3((40*i) as f32, 10.0 + (20*(i+1)) as f32, 0.0);
                    body_nodes[i].rotation = glm::vec3(path.roll, path.yaw, path.pitch);
                    //helicopter_body_node.rotation = glm::vec3(0.0, 3.14159265, 0.0);
    
                    (*body_nodes[i].children[1]).rotation = glm::vec3(0.0, elapsed * 2.0, 0.0);
                    (*body_nodes[i].children[2]).rotation = glm::vec3(elapsed * 8.0, 0.0, 0.0);   

                    // Ensure duration_since doesn't panic
                    if time_of_last_door_state_change < now {
                        let time_since_last_door_state_change = now.duration_since(time_of_last_door_state_change).as_secs_f32();
                        // Open door
                        if opening_door[i] {
                            if (*body_nodes[i].children[0]).position[2] < 2.0 {
                                (*body_nodes[i].children[0]).position += glm::vec3(0.0, 0.0, time_since_last_door_state_change*0.1);
                            }
                            else{
                                (*body_nodes[i].children[0]).position = glm::vec3(0.0, 0.0, 2.0);
                            }
                        }
                        else{
                            if (*body_nodes[i].children[0]).position[2] > 0.0 {
                                (*body_nodes[i].children[0]).position -= glm::vec3(0.0, 0.0, time_since_last_door_state_change*0.1);
                            }
                            else{
                                (*body_nodes[i].children[0]).position = glm::vec3(0.0, 0.0, 0.0);
                            }
                        }
                        
                    }




                }

                update_node_transformations(&mut lunar_node, &glm::identity());

                draw_scene(&lunar_node, &combined_transformation);
            }

            context.swap_buffers().unwrap();
        }
    });

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

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
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

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    },
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => { }
                }
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            },
            _ => { }
        }
    });
}