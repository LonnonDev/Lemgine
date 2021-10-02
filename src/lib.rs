mod event_loop;
mod renderer;
mod traits;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}