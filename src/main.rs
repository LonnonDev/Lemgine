use glium::implement_vertex;

use crate::{event_loop::event_loop_function, renderer::Renderer, traits::VectorUnnormalizedValues};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

mod event_loop;
mod traits;
mod renderer;

fn main() {
    implement_vertex!(Vertex, position);

    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb.clone(), cb, &event_loop).unwrap();

    let vertex1 = Vertex { position: [-300.0, -300.0] };
    let vertex2 = Vertex { position: [ 300.0,  300.0] };
    let vertex3 = Vertex { position: [ 300.0, -300.0] };
    let shape = vec![vertex1, vertex2, vertex3].unnormalize_values(wb);

    let renderer = Renderer::new().add_to_dynamic_from_vec(shape);

    event_loop_function(event_loop, display, renderer);
}