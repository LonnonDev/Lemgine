#![feature(destructuring_assignment)]
#![feature(in_band_lifetimes)]

use std::{fs};

use glium::{Surface, glutin::{event::{DeviceId, ElementState, KeyboardInput, VirtualKeyCode}}, implement_vertex, uniform};

use crate::{event_loop::{VecTuple, WindowDrawer}, renderer::Renderer, traits::VectorUnnormalizedValues};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

mod event_loop;
mod traits;
mod renderer; 

trait Rendering {
    fn render_cube(&mut self, variables: Vec<VecTuple>) -> Vec<VecTuple>;
}

impl Rendering for WindowDrawer {
    fn render_cube(&mut self, variables: Vec<VecTuple>) -> Vec<VecTuple> {
        let x = variables[1].unwrap::<f32>().clone();
        let y = variables[0].unwrap::<f32>().clone();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        self.size = self.display.gl_window().window().inner_size().to_logical::<f32>(1.0);
        self.renderer.update();
        self.renderer.add_to_dynamic_from_vec(&self.vertices);

        // Get the Vec<Vertex> Again
        self.shape_vertices = self.renderer.constant.clone();
        self.shape_vertices.append(&mut self.renderer.dynamic);
        self.shape_vertices = self.shape_vertices.unnormalize_values(self.size.clone());

        self.vertex_buffer = glium::VertexBuffer::new(&self.display, &self.shape_vertices).unwrap();
        self.program = glium::Program::from_source(&self.display, self.vertex_shader.as_str(), 
        self.fragment_shader.as_str(), None).unwrap();

        let mut target = self.display.draw();
        target.clear_color(255.0/255.0, 180.0/255.0, 180.0/255.0, 1.0);
        target.draw(&self.vertex_buffer, &indices, &self.program, &uniform!{ x: x, y: y }, &Default::default()).unwrap();
        target.finish().unwrap();
        return variables
    }
}

trait Input {
    fn get_movement(&mut self, variables: Vec<VecTuple>, device_id: DeviceId, input: KeyboardInput, is_synthetic: bool) -> Vec<VecTuple>; 
}

impl Input for WindowDrawer {
    fn get_movement(&mut self, mut variables: Vec<VecTuple>, _: DeviceId, input: KeyboardInput, _: bool) -> Vec<VecTuple> {
        let x = variables[0].unwrap::<f32>().clone();
        let y = variables[1].unwrap::<f32>().clone();
        if input.virtual_keycode == Some(VirtualKeyCode::W) && input.state == ElementState::Pressed {
            variables[0] = VecTuple::F32(x + (x + 10.0)/self.size.width);
        } if input.virtual_keycode == Some(VirtualKeyCode::S) && input.state == ElementState::Pressed {
            variables[0] = VecTuple::F32(x + (x - 10.0)/self.size.width);
        } if input.virtual_keycode == Some(VirtualKeyCode::D) && input.state == ElementState::Pressed {
            variables[1] = VecTuple::F32(y + (y + 10.0)/self.size.height);
        } if input.virtual_keycode == Some(VirtualKeyCode::A) && input.state == ElementState::Pressed {
            variables[1] = VecTuple::F32(y + (y - 10.0)/self.size.height);
        }

        return variables
    }
}

fn main() {
    implement_vertex!(Vertex, position);
    
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb.clone(), cb, &event_loop).unwrap();

    // Vertices of an object
    let vertex1 = Vertex { position: [-100.0, -100.0] };
    let vertex2 = Vertex { position: [ 100.0,  100.0] };
    let vertex3 = Vertex { position: [ 100.0, -100.0] };
    let vertex4 = Vertex { position: [ 100.0,  100.0] };
    let vertex5 = Vertex { position: [-100.0, -100.0] };
    let vertex6 = Vertex { position: [-100.0,  100.0] };
    let triangle = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    let vertex = fs::read_to_string("shader.vert")
        .expect("Something went wrong reading the file");
    let fragment = fs::read_to_string("shader.frag")
        .expect("Something went wrong reading the file");


    let window_drawer = WindowDrawer::new(vertex, fragment, display, triangle, Renderer::new());
    let variables = vec![VecTuple::F32(0f32), VecTuple::F32(0f32)];
    let function_test = Rendering::render_cube;
    let function_test2 = Input::get_movement;
    window_drawer.run(event_loop, variables, vec![function_test], vec![function_test2]);
}