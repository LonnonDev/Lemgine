use lemgine::backend::{glutin::event::{DeviceId, KeyboardInput}};
use lemgine::{vertex::Vertex, event_loop::{VecTuple, WindowDrawer}, renderer::Renderer};

trait Rendering {
    fn render_cube(&mut self, variables: Vec<VecTuple>) -> Vec<VecTuple>;
}

impl Rendering for WindowDrawer {
    fn render_cube(&mut self, variables: Vec<VecTuple>) -> Vec<VecTuple> {
        return variables
    }
}

trait Input {
    fn get_movement(&mut self, variables: Vec<VecTuple>, device_id: DeviceId, input: KeyboardInput, is_synthetic: bool) -> Vec<VecTuple>; 
}

impl Input for WindowDrawer {
    fn get_movement(&mut self, variables: Vec<VecTuple>, _: DeviceId, _: KeyboardInput, _: bool) -> Vec<VecTuple> {
        return variables
    }
}

fn main() {
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
    let vertex = "
#version 140

in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}    
".to_string();
    let fragment = "
#version 140

out vec4 color;

void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
}
".to_string();

    let window_drawer = WindowDrawer::new(vertex, fragment, display, triangle, Renderer::new());
    let variables = vec![];
    window_drawer.run(event_loop, variables, vec![Rendering::render_cube], vec![Input::get_movement], vec![]);
}