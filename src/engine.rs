use std::{any::Any, collections::HashMap, time::{Duration, Instant}};

use glium::{Display, Program, VertexBuffer, glutin::{self, dpi::LogicalSize, event_loop::{ControlFlow, EventLoop}}, implement_vertex, index::NoIndices};

use crate::{vertex::Vertex, renderer::Renderer, traits::VectorUnnormalizedValues};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Variable {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
}

impl Variable {
    pub fn b_unwrap(&self) -> Option<Box<dyn Any>> {
        match self.clone() {
            Variable::U8(val) => Some(Box::new(val)),
            Variable::U16(val) => Some(Box::new(val)),
            Variable::U32(val) => Some(Box::new(val)),
            Variable::U64(val) => Some(Box::new(val)),
            Variable::U128(val) => Some(Box::new(val)),
            Variable::I8(val) => Some(Box::new(val)),
            Variable::I16(val) => Some(Box::new(val)),
            Variable::I32(val) => Some(Box::new(val)),
            Variable::I64(val) => Some(Box::new(val)),
            Variable::I128(val) => Some(Box::new(val)),
            Variable::F32(val) => Some(Box::new(val)),
            Variable::F64(val) => Some(Box::new(val)),
        }
    }
    pub fn a_unwrap<T: 'static + Copy + Sized>(&self) -> T {
        let x = *self.b_unwrap().unwrap().downcast_ref::<T>().unwrap();
        return x
    }
    pub fn unwrap<T: 'static + Copy + Sized>(&self) -> T {
        let x = self.a_unwrap::<T>();
        return x
    }
}

trait StringInVec {
    fn in_vec(&self, value: String) -> bool;
}

impl StringInVec for Vec<String> {
    fn in_vec(&self, value: String) -> bool {
        for x in self {
            if *x == value {
                return true;
            }
        }
        return false;
    }
}

pub struct WindowDrawer {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub display: Display, 
    pub size: LogicalSize<f32>, 
    pub renderer: Renderer, 
    pub shape_vertices: Vec<Vertex>, 
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub program: Program,
    pub fps: f64,
    pub indices: NoIndices,
    variables: Vec<Variable>,
    variables_hash: HashMap<String, usize>,
}

impl WindowDrawer {
    pub fn new(
        vertex_shader: String, 
        fragment_shader: String, 
        display: Display, 
        renderer: Renderer, 
    ) -> WindowDrawer {
        implement_vertex!(Vertex, position);
        return WindowDrawer { 
            vertex_shader: vertex_shader.clone(),
            fragment_shader: fragment_shader.clone(),
            display: display.clone(), 
            size: display.gl_window().window().inner_size().to_logical::<f32>(1.0), 
            renderer, 
            shape_vertices: vec![], 
            vertex_buffer: glium::VertexBuffer::new(&display, &vec![]).unwrap(), 
            program: glium::Program::from_source(
                &display,
          vertex_shader.as_str(), 
        fragment_shader.as_str(), 
        None
            ).unwrap(), 
            fps: 60.0,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            variables: vec![],
            variables_hash: HashMap::new()
        }
    }

    pub fn get_var<T: 'static + Sized + Copy>(&self, name: &'static str) -> T {
        let index = self.variables_hash.get(name).unwrap();
        let return_value = self.variables[*index].unwrap();
        return return_value
    }

    pub fn add_variable(&mut self, name: &'static str, value: Variable) {
        self.variables_hash.insert(name.to_owned(), self.variables.len());
        self.variables.push(value);
    }

    pub fn mutate_var(&mut self, name: &'static str, value: Variable) {
        let index = self.variables_hash.get(name).unwrap();
        self.variables[*index] = value;
    }

    #[allow(unused_assignments)]
    pub fn run(
        mut self, 
        event_loop: EventLoop<()>, 
        rendering_functions: Vec<for<'r> fn(&'r mut Self) -> ()>,
        input_functions: Vec<for<'r> fn(&'r mut Self, glutin::event::DeviceId, glutin::event::KeyboardInput, bool) -> ()>,
        features: Vec<String>,
    ) {
        // Size of the screen
        self.size = self.display.gl_window().window().inner_size().to_logical::<f32>(1.0);

        // Vector of Vertices to use to draw stuff
        let mut shape_vertices = self.renderer.constant.clone();
        shape_vertices.append(&mut self.renderer.dynamic);
        shape_vertices = shape_vertices.unnormalize_values(self.size.clone());

        let mut frames = 0;

        let mut last_update = Instant::now();
        let mut last_second = Instant::now();

        event_loop.run(move |ev, _, control_flow| {  
            let next_frame_time = Instant::now() +
                std::time::Duration::from_nanos(1_000_000 / self.fps as u64);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    #[allow(unused_variables)]
                    glutin::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                        for x in input_functions.clone() {
                            x(&mut self, device_id, input, is_synthetic);
                        }
                    },
                    _ => return,
                },
                glutin::event::Event::RedrawRequested(_) => {
                    for x in rendering_functions.clone() {
                        x(&mut self);
                    }
                },
                glutin::event::Event::RedrawEventsCleared => {
                    let target_frametime = Duration::from_secs_f64(1.0 / self.fps);
                    let time_since_last_frame = last_update.elapsed();
                    if time_since_last_frame >= target_frametime {
                        frames += 1;
                        self.display.gl_window().window().request_redraw();
                        last_update = Instant::now();
                    } else {
                        *control_flow = ControlFlow::WaitUntil(
                            Instant::now() + target_frametime - time_since_last_frame,
                        );
                    }
                },
                _ => (),
            }
            let time_now = Instant::now();
            if time_now.duration_since(last_second) >= Duration::new(1,0) {
                if features.in_vec("framerate".to_string()) || features.in_vec("fps".to_string()) {
                    println!("{} FPS", frames);
                }
                last_second = Instant::now();
                frames = 0;
            }
        });
    }
}