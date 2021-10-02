use std::{any::Any, time::{Duration, Instant}};

use glium::{Display, Program, VertexBuffer, glutin::{self, dpi::LogicalSize, event_loop::{ControlFlow, EventLoop}}};

use crate::{Vertex, renderer::Renderer, traits::VectorUnnormalizedValues};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum VecTuple {
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

impl VecTuple {
    pub fn b_unwrap(&self) -> Option<Box<dyn Any>> {
        match self.clone() {
            VecTuple::U8(val) => Some(Box::new(val)),
            VecTuple::U16(val) => Some(Box::new(val)),
            VecTuple::U32(val) => Some(Box::new(val)),
            VecTuple::U64(val) => Some(Box::new(val)),
            VecTuple::U128(val) => Some(Box::new(val)),
            VecTuple::I8(val) => Some(Box::new(val)),
            VecTuple::I16(val) => Some(Box::new(val)),
            VecTuple::I32(val) => Some(Box::new(val)),
            VecTuple::I64(val) => Some(Box::new(val)),
            VecTuple::I128(val) => Some(Box::new(val)),
            VecTuple::F32(val) => Some(Box::new(val)),
            VecTuple::F64(val) => Some(Box::new(val)),
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

pub struct WindowDrawer {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub display: Display, 
    pub size: LogicalSize<f32>, 
    pub vertices: Vec<Vertex>, 
    pub renderer: Renderer, 
    pub shape_vertices: Vec<Vertex>, 
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub program: Program,
    pub fps: f64
}

impl WindowDrawer {
    pub fn new(
        vertex_shader: String, 
        fragment_shader: String, 
        display: Display, 
        vertices: Vec<Vertex>, 
        renderer: Renderer, 
    ) -> WindowDrawer {
        return WindowDrawer { 
            vertex_shader: vertex_shader.clone(),
            fragment_shader: fragment_shader.clone(),
            display: display.clone(), 
            size: display.gl_window().window().inner_size().to_logical::<f32>(1.0), 
            vertices, 
            renderer, 
            shape_vertices: vec![], 
            vertex_buffer: glium::VertexBuffer::new(&display, &vec![]).unwrap(), 
            program: glium::Program::from_source(
                &display,
          vertex_shader.as_str(), 
        fragment_shader.as_str(), 
        None
            ).unwrap(), 
            fps: 60.0
        }
    }

    #[allow(unused_assignments)]
    pub fn run(
        mut self, 
        event_loop: EventLoop<()>, 
        mut variables: Vec<VecTuple>, 
        rendering_functions: Vec<for<'r> fn(&'r mut Self, Vec<VecTuple>) -> Vec<VecTuple>>,
        input_functions: Vec<for<'r> fn(&'r mut Self, Vec<VecTuple>, glutin::event::DeviceId, glutin::event::KeyboardInput, bool) -> Vec<VecTuple>>,
    ) {
        println!("{:?}", variables);
        self.renderer.add_to_dynamic_from_vec(&self.vertices);

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
                            println!("{:?}", input);
                            variables = x(&mut self, variables.clone(), device_id, input, is_synthetic);
                        }
                    },
                    _ => return,
                },
                glutin::event::Event::RedrawRequested(_) => {
                    for x in rendering_functions.clone() {
                        variables = x(&mut self, variables.clone());
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
                println!("{} FPS", frames);
                last_second = Instant::now();
                frames = 0;
            }
        });
    }
}